use std::marker::PhantomData;

use crate::{Error, InputStream, Lookahead, Parser, ParserKind, Prefix};

/// Parser created by [`tag`] function.
pub struct Tag<T, I> {
    list: T,
    _marker: PhantomData<I>,
}

impl<T, I> Parser<I> for Tag<T, I>
where
    T: Prefix,
    I: InputStream,
{
    type Error = Error;
    type Output = ();

    fn parse(
        &mut self,
        mut input: I,
    ) -> impl std::future::Future<Output = crate::Result<I, Self::Output, Self::Error>> {
        async move {
            loop {
                if input.len() < self.list.len() {
                    if let Lookahead::BrokenPipe =
                        input.lookahead(self.list.len() - input.len()).await
                    {
                        return Err((input, Error::BrokenPipe(ParserKind::Tag)));
                    }

                    continue;
                }

                if self.list.find(input.slice()) {
                    return Ok((input.split_at(self.list.len()), ()));
                } else {
                    return Err((input, Error::ParseFailed(ParserKind::Tag)));
                }
            }
        }
    }
}

pub fn tag<T, I>(tag: T) -> Tag<T, I>
where
    I: InputStream,
    T: Prefix,
{
    Tag {
        list: tag,
        _marker: Default::default(),
    }
}

// #[cfg(test)]
// mod tests {
//     use futures::{executor::ThreadPool, task::SpawnExt};

//     use crate::{
//         combinator::{map, select},
//         Error, Parser, ParserKind,
//     };

//     use super::*;

//     #[futures_test::test]
//     async fn test_tag() {
//         let pool = ThreadPool::new().unwrap();

//         assert_eq!(
//             select((map(tag("xml"), |_| 1), map(tag("json"), |_| 2)))
//                 .parse("json hello")
//                 .await,
//             Ok((" hello", 2))
//         );

//         // pool.spawn_with_handle(async {
//         //     assert_eq!(
//         //         (tag("xml"), tag("json")).parse("json hello").await,
//         //         Ok((" hello", ((), ())))
//         //     );

//         //     // assert_eq!(tag("éhello").parse("éhello~~~").await, Ok(("~~~", ())));

//         //     // assert_eq!(
//         //     //     tag("éhello").parse("hello~~~").await,
//         //     //     Err(("hello~~~", Error::ParseFailed(ParserKind::Tag)))
//         //     // );

//         //     // assert_eq!(
//         //     //     select((map(tag("xml"), |_| 1), map(tag("json"), |_| 2)))
//         //     //         .parse("json hello")
//         //     //         .await,
//         //     //     Ok((" hello", 2))
//         //     // );

//         //     // assert_eq!(
//         //     //     select((map(tag("xml"), |_| 1), map(tag("json"), |_| 2)))
//         //     //         .parse("xml hello")
//         //     //         .await,
//         //     //     Ok((" hello", 1))
//         //     // );

//         //     // assert_eq!(
//         //     //     select((map(tag("xml"), |_| 1), map(tag("json"), |_| 2)))
//         //     //         .parse("a hello")
//         //     //         .await,
//         //     //     Err(("a hello", Error::ParseFailed(ParserKind::Tag)))
//         //     // );
//         // })
//         // .unwrap()
//         // .await;
//     }
// }
