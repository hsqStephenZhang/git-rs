use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_while},
    error::ParseError,
    multi::{many0, many_m_n},
    sequence::{delimited, preceded, terminated, tuple},
    IResult, Parser,
};

use crate::{
    index::{self, Index, IndexEntry},
    object::{
        commit::{AuthorInfo, Commit, CommitterInfo},
        Blob, Object, Tree, TreeEntry,
    },
    refs::Head,
    utils::bytes::{bytes_to_hex, bytes_to_string, bytes_to_usize, hex_to_i32},
};

pub fn decode_object<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let mut parser = alt((decode_blob, decode_tree, decode_commit));
    Ok(parser.parse(content)?)
}

pub fn decode_blob<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let parser = tag("blob ".as_bytes());
    let (content, _blob) = parser(content)?;
    let (content, size) = terminated(take_till(|c| c == b'\0'), tag(b"\0"))(content)?;

    let size = bytes_to_usize(size);
    let blob = Blob::new(content[0..size].to_owned());
    Ok((content, Object::Blob(blob)))
}

pub fn decode_tree<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let parser = tag("tree ".as_bytes());
    let (content, _tree) = parser(content)?;

    let (content, _size) = terminated(take_till(|c| c == b'\0'), tag(b"\0"))(content)?;

    let mode_parser = take_till(|c| c == b' ');
    let name_parser = delimited(tag(b" "), take_till(|c| c == b'\0'), tag(b"\0"));
    let hex_paser = take(20usize);

    let entry_parser = tuple((mode_parser, name_parser, hex_paser));
    let mut parser = many0(entry_parser);
    let (content, lines): (_, Vec<(&[u8], &[u8], &[u8])>) = parser(content)?;

    let mut entrys = Vec::with_capacity(lines.len());
    for (mode, filename, hex) in lines {
        let mode = mode.into();
        let filename = bytes_to_string(filename);
        let hex = bytes_to_hex(hex);
        let child = TreeEntry::new(mode, hex, filename);
        entrys.push(child);
    }
    Ok((content, Object::Tree(Tree::new(entrys))))
}

pub fn decode_commit<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&[u8], Object, E> {
    let parser = tag(b"commit ");
    let (content, _commit) = parser(content)?;

    let (content, _size) = terminated(take_till(|c| c == b'\0'), tag(b"\0"))(content)?;

    let mut tree_parser = tuple((
        tag(b"tree"),
        delimited(tag(b" "), take(40usize), tag(b"\n")),
    ));
    let (content, tree_attr) = tree_parser(content)?;

    let hex = tree_attr.1;
    let root_sha1 = bytes_to_string(hex);

    let parent_entry_parser = tuple((
        tag(b"parent"),
        delimited(tag(b" "), take(40usize), tag(b"\n")),
    ));
    let mut parents_parser = many0(parent_entry_parser);
    let (content, parent_attrs): (_, Vec<(&[u8], &[u8])>) = parents_parser(content)?;

    let parents_sha1 = if parent_attrs.len() == 0 {
        None
    } else {
        let v = parent_attrs
            .iter()
            .map(|(_, hex)| bytes_to_string(hex))
            .collect();
        Some(v)
    };

    // author hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n
    // committer hsqStephenZhang <2250015961@qq.com> 1638597231 +0000\n\nadd tree parse test\n
    // \nmessage

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

pub fn decode_index_entry<'a, E: ParseError<&'a [u8]>>(
    content: &'a [u8],
) -> IResult<&'a [u8], IndexEntry, E> {
    use nom::number::complete::u16 as p_u16;
    let p_u16 = p_u16(nom::number::Endianness::Big);
    use nom::number::complete::u32 as p_u32;
    let p_u32 = p_u32(nom::number::Endianness::Big);

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
        filepath.iter().map(|&v|v as char).collect(),
        padding.len(),
    );

    Ok((content, entry))
}

pub fn decode_tree_extension<'a, E: ParseError<&'a [u8]>>(
    content: &'a [u8],
) -> IResult<&'a [u8], index::TreeExtension, E> {
    use nom::number::complete::u32 as p_u32;
    let p_u32 = p_u32(nom::number::Endianness::Big);
    let tree = tag(b"TREE");
    let length = p_u32;

    // ignore the content length here
    let (content, (_tree, _, root)) =
        tuple((tree, length, decode_tree_extension_subtree))(content)?;

    return Ok((content, root));
}

pub fn decode_tree_extension_subtree<'a, E: ParseError<&'a [u8]>>(
    content: &'a [u8],
) -> IResult<&'a [u8], index::TreeExtension, E> {
    let path = take_till(|c| c == b'\0');
    let entry_num_parser = preceded(tag(b"\0"), take_till(|c| c == b' '));
    let subtree_num_parser = delimited(tag(b" "), take_till(|c| c == b'\n'), tag(b"\n"));
    let mut tree_meta_parser = tuple((path, entry_num_parser, subtree_num_parser));
    let (content, (path, entry_num, subtree_num)) = tree_meta_parser(content)?;

    let path = String::from_utf8(path.to_owned()).unwrap();
    let entry_num = hex_to_i32(entry_num);
    let subtree_num = hex_to_i32(subtree_num);

    let (mut content, hex) = if entry_num != -1 {
        let hex_parser = take(20usize);
        let (content, hex) = hex_parser(content)?;
        (content, Some(hex.to_owned()))
    } else {
        (content, None)
    };

    let mut subtrees = Vec::with_capacity(subtree_num as usize);

    for _ in 0..subtree_num as usize {
        let (__content, subtree) = decode_tree_extension_subtree(content)?;
        content = __content;
        subtrees.push(subtree);
    }

    return Ok((
        content,
        index::TreeExtension::new(path, entry_num, subtree_num, hex, subtrees),
    ));
}

pub fn decode_index<'a, E: ParseError<&'a [u8]>>(content: &'a [u8]) -> IResult<&'a [u8], Index, E> {
    use nom::number::complete::u32 as p_u32;
    let p_u32 = p_u32(nom::number::Endianness::Big);
    let mut parser = tuple((p_u32, p_u32, p_u32));
    let (content, (dirc, version, num_entrys)) = parser(content)?;

    let mut entrys_parser = many_m_n(num_entrys as usize, num_entrys as usize, decode_index_entry);
    let checksum_parser = take(20usize);

    let (content, entrys) = entrys_parser(content)?;

    let (content, tree_extension) = if content.len() >= 4 && &content[..4] == b"TREE" {
        let (content, tree_extension) = decode_tree_extension(content)?;
        (content, Some(tree_extension))
    } else {
        (content, None)
    };

    let (content, checksum) = checksum_parser(content)?;

    let index = Index::new(
        dirc,
        version,
        num_entrys,
        entrys,
        checksum.into(),
        tree_extension,
    );
    Ok((content, index))
}

// 1. ref: refs/heads/master
// 2. 48b3d19840c917d8e5990ae629273a0a1cd2b606
pub fn decode_head_pointer<'a, E: ParseError<&'a [u8]>>(
    content: &'a [u8],
) -> IResult<&'a [u8], Head, E> {
    let ref_parser = tuple((tag(b"ref: "), take_till(|c| c == b'\0' || c == b'\n'))).map(
        |(_, res): (&[u8], &[u8])| {
            let s = String::from_utf8(res.into()).unwrap();
            Head::Ref(s)
        },
    );
    let sha1_parser = take(20usize).map(|res: &[u8]| Head::Pointer(res.into()));

    let mut head_parser = alt((ref_parser, sha1_parser));
    let (content, head) = head_parser.parse(content)?;
    Ok((content, head))
}

#[cfg(test)]
mod tests {
    use nom::{AsBytes, IResult};

    use crate::{
        parser::decode::{decode_commit, decode_object, decode_tree},
        utils::sha1::decode_file,
    };

    use super::{decode_blob, decode_head_pointer, decode_index};

    #[test]
    fn test_blob_decode_encode() {
        // 3b18e512dba79e4c8300dd08aeb37f8e728b8dad
        let content = decode_file("data/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad");
        let content = content.as_bytes();

        let r: IResult<_, _> = decode_blob(content);
        let (_, obj) = r.unwrap();
        dbg!(&obj);

        let r: Vec<u8> = Into::into(&obj);
        println!("encode:{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_tree_decode_encode() {
        // 8465cd187d9bad9e5a7931c2119f16311f9923a7
        let content = decode_file("data/objects/84/65cd187d9bad9e5a7931c2119f16311f9923a7");
        let content = content.as_bytes();
        // println!("content:{:?}\n\n", content);

        let r: IResult<_, _> = decode_tree(content);
        let (_, obj) = r.unwrap();
        dbg!(&obj);

        let r: Vec<u8> = Into::into(&obj);
        println!("encode:{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_tree_decode_encode2() {
        //3bf04fcd182b02137c1d0e9946a2c77ec9ac1164
        let content = decode_file("data/objects/3b/f04fcd182b02137c1d0e9946a2c77ec9ac1164");
        let content = content.as_bytes();
        // println!("content:{:?}\n\n", content);

        let r: IResult<_, _> = decode_tree(content);
        let (_, obj) = r.unwrap();
        dbg!(&obj);

        let r: Vec<u8> = Into::into(&obj);
        println!("encode:{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_commit_decode_encode() {
        // ef074b7c01f72b2a16eea122c90035ff7649d855
        let content = decode_file("data/objects/ef/074b7c01f72b2a16eea122c90035ff7649d855");
        let content = content.as_bytes();
        // println!("content:{:?}\n\n", content);

        let r: IResult<_, _> = decode_commit(content);
        let (_, obj) = r.unwrap();
        dbg!(&obj);

        let r: Vec<u8> = Into::into(&obj);
        // println!("encode :{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_commit_decode_encode2() {
        // 6f4f12edec17b6c97795f14fe703fc680111ad83
        let content = decode_file("data/objects/6f/4f12edec17b6c97795f14fe703fc680111ad83");
        let content = content.as_bytes();
        // println!("content:{:?}\n\n", content);

        let r: IResult<_, _> = decode_commit(content);
        let (_, obj) = r.unwrap();

        dbg!(&obj);

        let r: Vec<u8> = Into::into(&obj);
        // println!("encode :{:?}", r);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_object_decode() {
        // 6f4f12edec17b6c97795f14fe703fc680111ad83
        let content = decode_file("data/objects/6f/4f12edec17b6c97795f14fe703fc680111ad83");
        let content = content.as_bytes();

        let r: IResult<_, _> = decode_object(content);
        let (_, obj) = r.unwrap();

        let r: Vec<u8> = Into::into(&obj);

        assert_eq!(content, &r[..]);
    }

    #[test]
    fn test_decode_index() {
        let content = std::fs::read("data/index").unwrap();
        let content = content.as_bytes();

        let r: IResult<_, _> = decode_index(content);
        let r = r.unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_decode_index2() {
        let content = std::fs::read("data/index2").unwrap();
        let content = content.as_bytes();

        let r: IResult<_, _> = decode_index(content);
        let r = r.unwrap();
        println!("{:?}", r);
    }

    #[test]
    fn test_decode_index3() {
        let content = std::fs::read(".git/index").unwrap();
        let content = content.as_bytes();

        let r: IResult<_, _> = decode_index(content);
        let r = r.unwrap();
        println!("{:?}", r);
    }
    #[test]
    fn test_decode_header() {
        // 1. ref: refs/heads/master
        // 2. 48b3d19840c917d8e5990ae629273a0a1cd2b606
        let content = &b"ref: refs/heads/master"[..];
        let r: IResult<_, _> = decode_head_pointer(content);
        let (_, head) = r.unwrap();
        println!("{:?}", head);

        let content = &b"48b3d19840c917d8e5990ae629273a0a1cd2b606"[..];
        let r: IResult<_, _> = decode_head_pointer(content);
        let (_, head) = r.unwrap();
        println!("{:?}", head);
    }
}
