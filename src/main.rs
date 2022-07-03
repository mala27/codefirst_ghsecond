// Chapter 3 Rust Language Book Challenge: Christmas Carol, opted for Ain't No Sunshine by Bill Withers instead 

fn main() {
    let song = ["It's not warm when she's away",
                "And she's always gone too long",
                "Anytime she goes away",
                "Wonder if she's gone to stay",
                "And this house ain't no home",
                "Anytime she goes away",
                "I know, I know, I know, I know",
                "I know, I know, I know, I know",
                "Hey I oughta leave young thing alone",
                "Only darkness everyday",
                "And this house ain't no home",
                "Anytime she goes away"];
                
    
    for lyric in 0..3 {
    println!("Ain't no sunshine when she's gone\n {}", song[lyric]);
    }
    println!(" ");
    
    for lyric in 3..6 {
    println!("Wonder this time where she's gone\n {}", song[lyric]);
    }
    println!(" ");     
        
    for lyric in 6..9 {
    println!("I know, I know, I know, I know\n {}", song[lyric]);
    }
    println!(" ");     

    for lyric in 9..12 {
    println!("Ain't no sunshine when she's gone\n {}", song[lyric]);
    }
    println!(" ");     
  
}
