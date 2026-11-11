; Encode and render ordered eq.
i_ordered_eq:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    sete al
    setnp dl
    and al,dl
    movzx eax,al
.bytes_end:
.text:
    db '    sete al',10
    db '    setnp dl',10
    db '    and al,dl',10
    db '    movzx eax,al',10
.text_end:
; Encode and render unordered ne.
i_unordered_ne:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    setne al
    setp dl
    or al,dl
    movzx eax,al
.bytes_end:
.text:
    db '    setne al',10
    db '    setp dl',10
    db '    or al,dl',10
    db '    movzx eax,al',10
.text_end:
; Encode and render ordered lt.
i_ordered_lt:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    setb al
    setnp dl
    and al,dl
    movzx eax,al
.bytes_end:
.text:
    db '    setb al',10
    db '    setnp dl',10
    db '    and al,dl',10
    db '    movzx eax,al',10
.text_end:
; Encode and render ordered le.
i_ordered_le:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    setbe al
    setnp dl
    and al,dl
    movzx eax,al
.bytes_end:
.text:
    db '    setbe al',10
    db '    setnp dl',10
    db '    and al,dl',10
    db '    movzx eax,al',10
.text_end:
; Encode and render ordered gt.
i_ordered_gt:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    seta al
    movzx eax,al
.bytes_end:
.text:
    db '    seta al',10
    db '    movzx eax,al',10
.text_end:
; Encode and render ordered ge.
i_ordered_ge:
    dd .bytes_end-.bytes,.text_end-.text
.bytes:
    setae al
    movzx eax,al
.bytes_end:
.text:
    db '    setae al',10
    db '    movzx eax,al',10
.text_end:
