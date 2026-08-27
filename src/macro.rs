#[macro_export]
macro_rules! animotion_clip {
    (
        $key:expr,
        $( 
            $name:ident : $init:expr $( => tween ( $target:expr, $dur:expr ) )+ 
        ),+ $(,)? ,
        |$el:ident, ( $( $arg:ident ),+ )| $body:expr
    ) => {
        $crate::AnimotionClipExt::animotion_clip(
            gpui::div(),
            $key,
            |c| {
                $(
                    let $name = c.prop($init);
                    $(
                        $name.tween($target, $dur);
                    )+
                )+
                ( $( $name ),+ )
            },
            |$el, ( $( $name ),+ )| {
                let ( $( $arg ),+ ) = ( $( $name.get() ),+ );
                $body
            }
        )
    };
}
