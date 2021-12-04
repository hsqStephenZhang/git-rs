use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till},
    error::{Error, ParseError},
    multi::many0,
    sequence::tuple,
    IResult, Parser,
};

use crate::{
    object::{
        commit::{AuthorInfo, Commit, CommitterInfo},
        Blob, Object, Tree, TreeEntry,
    },
    utils::bytes::{bytes_to_hex, bytes_to_usize},
};

pub fn decode(content: &[u8]) -> Result<Object, nom::Err<Error<&[u8]>>> {
    let mut parser = alt((decode_blob, decode_tree, decode_commit));
    let r = parser.parse(content)?;
    Ok(r.1)
}

fn decode_blob<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let parser = tag("blob".as_bytes());
    // let r: IResult<_, _> = parser(content);
    let (left, content) = parser(content)?;
    let blob = Blob::new(content.to_owned());
    Ok((left, Object::Blob(blob)))
}

fn decode_tree<'a, E: ParseError<&'a [u8]>>(content: &[u8]) -> IResult<&[u8], Object, E> {
    let parser = tag("tree".as_bytes());
    let r: IResult<_, _> = parser(content);
    let (content, _tree) = r.unwrap();
    let content = &content[1..];

    let parser = take_till(|c| c == b'\0');
    let r: IResult<_, _> = parser(content);
    let (content, _size) = r.unwrap();
    let content = &content[1..]; // skip '\0'

    let mode_parser = take(6usize);
    let nope = tag(b" ");
    let name_parser = take_till(|c| c == b'\0');
    let nope2 = tag(b"\0");
    let hex_paser = take(20usize);

    let entry_parser = tuple((mode_parser, nope, name_parser, nope2, hex_paser));
    let mut parser = many0(entry_parser);
    let res: IResult<_, Vec<(&[u8], _, &[u8], _, &[u8])>> = parser(content);
    let (content, lines) = res.unwrap();
    // println!("{:?}", r);
    let mut entrys = Vec::with_capacity(lines.len());
    for (mode, _, filename, _, hex) in lines {
        let hex = bytes_to_hex(hex);
        let mode = mode.into();
        let filename = String::from_utf8(filename.into()).unwrap();

        let child = TreeEntry::new(mode, hex, filename);
        entrys.push(child);
    }
    Ok((content, Object::Tree(Tree::new(entrys))))
}

fn decode_commit<'a, E: ParseError<&'a [u8]>>(content: &[u8]) -> IResult<&[u8], Object, E> {
    let parser = tag(b"commit");
    let r: IResult<_, _> = parser(content);
    let (content, _commit) = r.unwrap();

    let parser = take_till(|c| c == b'\0');
    let r: IResult<_, _> = parser(content);
    let (content, _size) = r.unwrap();
    let content = &content[1..]; // skip '\0'

    let mut tree_parser = tuple((tag(b"tree"), tag(b" "), take(40usize), tag(b"\n")));
    let r: IResult<_, _> = tree_parser(content);
    let (content, tree_attr) = r.unwrap();

    let hex = tree_attr.2;
    let root_sha1 = bytes_to_hex(hex);

    let parent_entry_parser = tuple((tag(b"parent"), tag(b" "), take(40usize), tag(b"\n")));
    let mut parents_parser = many0(parent_entry_parser);
    let r: IResult<_, Vec<(&[u8], _, &[u8], _)>> = parents_parser(content);
    let (content, parent_attrs) = r.unwrap();

    let parents_sha1 = if parent_attrs.len() == 0 {
        None
    } else {
        let v = parent_attrs
            .iter()
            .map(|(_, _, hex, _)| bytes_to_hex(hex))
            .collect();
        Some(v)
    };

    // author hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n
    // committer hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n\nadd tree parse test\n
    // \nadd tree parse test\n

    let raw_infos = String::from_utf8(content.to_vec()).unwrap();
    let infos: Vec<_> = raw_infos.splitn(4, "\n").collect();
    debug_assert!(infos.len() == 4);

    let raw_author = infos[0];
    debug_assert!(raw_author.starts_with("author"));
    let items = raw_author.split(" ").collect::<Vec<_>>();
    debug_assert!(items.len() == 5);

    let author_info = Some(AuthorInfo::new(
        items[1].into(),
        items[2].into(),
        bytes_to_usize(items[3].as_bytes()),
        items[4].into(),
    ));

    let raw_commiter = infos[1];
    debug_assert!(raw_commiter.starts_with("committer"));
    let items = raw_commiter.split(" ").collect::<Vec<_>>();
    debug_assert!(items.len() == 5);

    let commiter_info = Some(CommitterInfo::new(
        items[1].into(),
        items[2].into(),
        bytes_to_usize(items[3].as_bytes()),
        items[4].into(),
    ));

    let message = infos[3].into();

    let commit = Commit::new(root_sha1, parents_sha1, author_info, commiter_info, message);
    Ok(("".as_bytes(), Object::Commit(commit)))
}

#[cfg(test)]
mod tests {
    use nom::{
        bytes::complete::{tag, take, take_till},
        multi::many0,
        sequence::tuple,
        AsBytes, IResult,
    };

    use crate::{object::Blob, utils::sha1::decode_file};

    #[test]
    fn t1() {
        let content = decode_file(".git/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad");
        let content = content.as_bytes();

        let parser = tag("blob".as_bytes());
        let r: IResult<_, _> = parser(content);
        let r = r.unwrap();

        let blob = Blob::new(r.1.to_owned());
        println!("{:?}", blob);
    }

    #[test]
    fn t2() {
        let content = decode_file(".git/objects/84/65cd187d9bad9e5a7931c2119f16311f9923a7");
        let content = content.as_bytes();

        let parser = tag("tree".as_bytes());
        let r: IResult<_, _> = parser(content);
        let (content, name) = r.unwrap();
        let content = &content[1..];
        println!("{:?}", name);

        let parser = take_till(|c| c == b'\0');
        let r: IResult<_, _> = parser(content);
        let (content, size) = r.unwrap();
        let content = &content[1..]; // skip '\0'
        println!("{:?}", size);

        let mode_parser = take(6usize);
        let nope = tag(b" ");
        let name_parser = take_till(|c| c == b'\0');
        let nope2 = tag(b"\0");
        let hex_paser = take(20usize);

        let entry_parser = tuple((mode_parser, nope, name_parser, nope2, hex_paser));
        let mut parser = many0(entry_parser);
        let res: IResult<_, Vec<(&[u8], _, &[u8], _, &[u8])>> = parser(content);
        let entrys = res.unwrap().1;
        // println!("{:?}", r);
        for item in entrys {
            // let hexcode = item.4;
            // let s=String::from_utf8(hexcode.to_vec()).unwrap();
            // println!("{:?}", hexcode);
            // let s = bytes_to_hex(hexcode);
            println!("{:?}", item);
        }
    }

    #[test]
    fn test_commit() {
        // ef074b7c01f72b2a16eea122c90035ff7649d855
        let content = decode_file(".git/objects/ef/074b7c01f72b2a16eea122c90035ff7649d855");
        let content = content.as_bytes();
        let s = String::from_utf8(content.to_vec());
        println!("{:?}", s);

        let parser = tag("commit".as_bytes());
        let r: IResult<_, _> = parser(content);
        let (content, name) = r.unwrap();
        println!("{:?}", name);

        let parser = take_till(|c| c == b'\0');
        let r: IResult<_, _> = parser(content);
        let (content, _) = r.unwrap();
        let content = &content[1..]; // skip '\0'

        let mut tree_parser = tuple((tag(b"tree"), tag(b" "), take(40usize), tag(b"\n")));
        let r: IResult<_, _> = tree_parser(content);
        let (content, tree_attr) = r.unwrap();
        println!("{:?}", tree_attr);

        let parent_entry_parser = tuple((tag(b"parent"), tag(b" "), take(40usize), tag(b"\n")));
        let mut parents_parser = many0(parent_entry_parser);
        let r: IResult<_, Vec<(&[u8], &[u8], &[u8], &[u8])>> = parents_parser(content);
        let (content, parent_attrs) = r.unwrap();
        for parent_attr in parent_attrs {
            println!("{:?}", parent_attr);
        }

        // author hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n
        // committer hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n\nadd tree parse test\n
        // \nadd tree parse test\n

        let raw_infos = String::from_utf8(content.to_vec()).unwrap();
        let infos = raw_infos.split("\n");
        for info in infos.clone().take(2) {
            println!("{}", info);

            if info.starts_with("author") {
                println!("author");
                let item = info.split(" ");
                for i in item {
                    println!("{}", i);
                }
            } else if info.starts_with("committer") {
                println!("committer");
                let item = info.split(" ");
                for i in item {
                    println!("{}", i);
                }
            }
        }

        for info in infos.skip(3) {
            println!("commit msg:{}", info);
        }
    }
}
