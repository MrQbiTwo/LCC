// Generate named functions with unsigned arithmetic, branches, loops and calls.
pub fn corpus(functions: usize) -> (String, i32) {
    if functions == 0 {
        return ("int main(void){return 37;}\n".into(), 37);
    }
    let mut source = String::from("#define rounds 8\n");
    for n in 0..functions {
        source.push_str(&format!("unsigned f{n:05}(unsigned x)\n{{\n    unsigned i;\n    for(i=0;i<rounds;i++){{\n        x=x*1664525u+{n}u+i;\n        if(x&1u) x^=x>>7;\n        else x+=x>>3;\n    }}\n    return x;\n}}\n"));
    }
    source.push_str("int main(int argc,char **argv)\n{\n    unsigned x=(unsigned)argc;\n");
    let mut value = 1u32;
    for n in 0..functions {
        source.push_str(&format!("    x=f{n:05}(x);\n"));
        for i in 0..8 {
            value = value
                .wrapping_mul(1664525)
                .wrapping_add(n as u32)
                .wrapping_add(i);
            if value & 1 != 0 {
                value ^= value >> 7;
            } else {
                value = value.wrapping_add(value >> 3);
            }
        }
    }
    source.push_str("    return (int)(x&255u);\n}\n");
    (source, (value & 255) as i32)
}
