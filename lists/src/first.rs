#[derive( Clone )]
pub struct List {
    head: Link,
}

#[derive( Clone, Debug )]
enum Link {
    Empty,
    More( Box<Node> ),
}

#[derive( Clone, Debug )]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // 直接把整个List复制
    pub fn push( &mut self, elem: i32 ) {
        let new_node = Box::new( Node {
            elem: elem,
            next: self.head.clone(),
        } );

        self.head = Link::More( new_node );
    }

    // 性能更好的复制
    pub fn push1( &mut self, elem: i32 ) {
        let new_node = Box::new( Node {
            elem: elem,
            next: std::mem::replace( &mut self.head, Link::Empty ),
        } );

        self.head = Link::More( new_node );
    }

    pub fn pop( &mut self ) -> Option<i32> {
        match std::mem::replace( &mut self.head, Link::Empty ) {
            Link::Empty => None,
            Link::More( node ) => {
                self.head = node.next;
                Some( node.elem )
            }
        }
        // unimplemented!();
    }
}

impl Drop for List {
    fn drop( &mut self ) {
        let mut cur_link = std::mem::replace( &mut self.head, Link::Empty );

        match cur_link {
            Link::More( ref node ) => {
                println!( "{}", node.elem );
            }
            _ => {},
        }

        
        while let Link::More( mut boxed_node ) = cur_link {
            cur_link = std::mem::replace( &mut boxed_node.next, Link::Empty );
            // boxed_node 在这里超出作用域并被 drop,
            // 由于它的 `next` 字段拥有的 `Node` 被设置为 Link::Empty,
            // 因此这里并不会有无边界的递归发生
            match cur_link {
                Link::More( ref node ) => {
                    println!( "{}", node.elem );
                }
                _ => {},
            }
        }
    }
}


#[cfg( test )]
mod test {
    #[test]
    fn basics() {
        // 命令行加上-- --nocapture 才会输出
        use super::List;
        let mut list = List::new();

        assert_eq!( list.pop(), None );

        list.push( 1 );
        list.push( 2 );
        list.push( 3 );

        assert_eq!( list.pop(), Some( 3 ) );
        assert_eq!( list.pop(), Some( 2 ) );

        list.push( 4 );
        list.push( 5 );

        assert_eq!( list.pop(), Some( 5 ) );
        assert_eq!( list.pop(), Some( 4 ) );

        assert_eq!( list.pop(), Some( 1 ) );
        assert_eq!( list.pop(), None );
    }

    #[test]
    fn long_list() {
        use super::List;

        // 测试堆栈是否会撑爆
        let mut list = List::new();
        for i in 0..100 {
            list.push( i );
        }

        drop( list );
    }
}
