use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_while},
    error::{Error, ParseError},
    multi::{many0, many_m_n},
    sequence::tuple,
    IResult, Parser,
};

use crate::{
    index::{self, Index, IndexEntry},
    object::{
        commit::{AuthorInfo, Commit, CommitterInfo},
        Blob, Object, Tree, TreeEntry,
    },
    utils::bytes::{bytes_to_hex, bytes_to_string, bytes_to_usize},
};

pub fn decode(content: &[u8]) -> Result<Object, nom::Err<Error<&[u8]>>> {
    let mut parser = alt((decode_blob, decode_tree, decode_commit));
    let r = parser.parse(content)?;
    Ok(r.1)
}

fn decode_blob<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let parser = tag("blob ".as_bytes());
    // let r: IResult<_, _> = parser(content);
    let (content, _blob) = parser(content)?;

    let parser = take_till(|c| c == b'\0');
    let (content, size) = parser(content)?;
    let content = &content[1..]; // skip '\0'

    let size = bytes_to_usize(size);

    let blob = Blob::new(content[0..size].to_owned());
    Ok((content, Object::Blob(blob)))
}

fn decode_tree<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let parser = tag("tree ".as_bytes());
    // let r: IResult<_, _> = parser(content);
    let (content, _tree) = parser(content)?;

    let parser = take_till(|c| c == b'\0');
    let (content, _size) = parser(content)?;
    let content = &content[1..]; // skip '\0'

    let mode_parser = take(6usize);
    let nop = tag(b" ");
    let name_parser = take_till(|c| c == b'\0');
    let nop2 = tag(b"\0");
    let hex_paser = take(20usize);

    let entry_parser = tuple((mode_parser, nop, name_parser, nop2, hex_paser));
    let entry_parser2 = tuple((
        tag(b"40000"),
        tag(b" "),
        take_till(|c| c == b'\0'),
        tag(b"\0"),
        take(20usize),
    ));
    let mut parser = many0(alt((entry_parser, entry_parser2)));
    let (content, lines): (_, Vec<(&[u8], _, &[u8], _, &[u8])>) = parser(content)?;
    let mut entrys = Vec::with_capacity(lines.len());
    for (mode, _, filename, _, hex) in lines {
        let mode = mode.into();
        let filename = bytes_to_string(filename);
        let hex = bytes_to_hex(hex);

        let child = TreeEntry::new(mode, hex, filename);
        entrys.push(child);
    }
    Ok((content, Object::Tree(Tree::new(entrys))))
}

fn decode_commit<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let parser = tag(b"commit ");
    let (content, _commit) = parser(content)?;

    let parser = take_till(|c| c == b'\0');
    let (content, _size) = parser(content)?;
    let content = &content[1..]; // skip '\0'

    let mut tree_parser = tuple((tag(b"tree"), tag(b" "), take(40usize), tag(b"\n")));
    let (content, tree_attr) = tree_parser(content)?;

    let hex = tree_attr.2;
    let root_sha1 = bytes_to_string(hex);

    let parent_entry_parser = tuple((tag(b"parent"), tag(b" "), take(40usize), tag(b"\n")));
    let mut parents_parser = many0(parent_entry_parser);
    let (content, parent_attrs): (_, Vec<(&[u8], _, &[u8], _)>) = parents_parser(content)?;

    let parents_sha1 = if parent_attrs.len() == 0 {
        None
    } else {
        let v = parent_attrs
            .iter()
            .map(|(_, _, hex, _)| bytes_to_string(hex))
            .collect();
        Some(v)
    };

    // author hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n
    // committer hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n\nadd tree parse test\n
    // \nadd tree parse test\n

    let raw_infos = bytes_to_string(content);
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

fn decode_index_entry<'a, E: ParseError<&'a [u8]>>(
    content: &'a [u8],
) -> IResult<&'a [u8], IndexEntry, E> {
    use nom::number::complete::u16 as p_u16;
    let p_u16 = p_u16(nom::number::Endianness::Big);
    use nom::number::complete::u32 as p_u32;
    let p_u32 = p_u32(nom::number::Endianness::Big);

    // use nom::number::complete::u64 as p_u64;
    // let p_u64 = p_u64(nom::number::Endianness::Big);

    use nom::number::complete::i64 as p_i64;
    let p_i64 = p_i64(nom::number::Endianness::Big);

    let mut parser = tuple((
        p_i64,
        p_i64,
        p_u32,
        p_u32,
        p_u32,
        p_u32,
        p_u32,
        p_u32,
        take(20usize),
        p_u16,
    ));

    let (content, (ctime, mtime, dev, kino, mode, uid, gid, files, hex, flags)) = parser(content)?;

    let (content, filepath) = take_till(|c| c == b'\0')(content)?;
    let (content, padding) = take_while(|c| c == b'\0')(content)?;

    let entry = IndexEntry::new(
        ctime,
        mtime,
        dev,
        kino,
        mode,
        uid,
        gid,
        files,
        hex.into(),
        flags,
        filepath.into(),
        padding.len(),
    );

    Ok((content, entry))
}

fn decode_index_extension<'a, E: ParseError<&'a [u8]>>(
    content: &'a [u8],
) -> IResult<&'a [u8], index::Extension, E> {
    // let tree = tag(b"TREE");

    todo!()
}

pub fn decode_index<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&'a [u8], Index, E> {
    use nom::number::complete::u32 as p_u32;
    let p_u32 = p_u32(nom::number::Endianness::Big);
    let mut parser = tuple((p_u32, p_u32, p_u32));
    let (content, (dirc, version, num_entrys)) = parser(content)?;

    let mut entrys = many_m_n(num_entrys as usize, num_entrys as usize, decode_index_entry);
    let hex_parser = take(20usize);

    let (content, entrys) = entrys(content)?;

    let (content, extension, hex) = if content.len() >= 4 && &content[..4] == b"TREE" {
        // TODO: parse extension and hex
        (content, None, &b""[..])
    } else {
        let (content, hex) = hex_parser(content)?;
        (content, None, hex)
    };

    let index = Index::new(dirc, version, num_entrys, entrys, hex.into(), extension);
    Ok((content, index))
}

#[cfg(test)]
mod tests {
    use nom::{AsBytes, IResult};

    use crate::{
        parser::decode::{decode_commit, decode_tree},
        utils::sha1::decode_file,
    };

    use super::{decode_blob, decode_index};

    #[test]
    fn test_blob_decode_encode() {
        // 3b18e512dba79e4c8300dd08aeb37f8e728b8dad
        let content = decode_file(".git/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad");
        let content = content.as_bytes();

        let r: IResult<_, _> = decode_blob(content);
        let (_, obj) = r.unwrap();

        let r: Vec<u8> = Into::into(&obj);
        // println!("encode:{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_tree_decode_encode() {
        // 8465cd187d9bad9e5a7931c2119f16311f9923a7
        let content = decode_file(".git/objects/84/65cd187d9bad9e5a7931c2119f16311f9923a7");
        let content = content.as_bytes();
        // println!("content:{:?}\n\n", content);

        let r: IResult<_, _> = decode_tree(content);
        let (_, obj) = r.unwrap();

        let r: Vec<u8> = Into::into(&obj);
        // println!("encode:{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_commit_decode_encode() {
        // ef074b7c01f72b2a16eea122c90035ff7649d855
        let content = decode_file(".git/objects/ef/074b7c01f72b2a16eea122c90035ff7649d855");
        let content = content.as_bytes();
        // println!("content:{:?}\n\n", content);

        let r: IResult<_, _> = decode_commit(content);
        let (_, obj) = r.unwrap();

        let r: Vec<u8> = Into::into(&obj);
        // println!("encode :{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_decode_index() {
        let content = std::fs::read("data/index").unwrap();
        let content = content.as_bytes();

        let r: IResult<_, _> = decode_index(content);
        let (_, index) = r.unwrap();
        println!("{:?}", index);
    }
}
