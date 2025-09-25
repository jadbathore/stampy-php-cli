<?php
if (php_sapi_name() !== 'cli') {
    exit("Ce script doit être exécuté en ligne de commande.\n");
}

if (getenv("LIB_STAMPY")) {
    echo PHP_EOL;
    $select = ["continue as such","use docker"];
    $input = Dialoguer::select("the stampy extension add already a pré-compile binairy you can  ? ",
    $select,true);
    if ($input == $select[1]){
        exit(1);
    }

} else {
$clearCount=3;
echo "the stampy extension add no pré-compile binairy for your architecture you can compile the binairy by yourself 
using cargo or use docker.If you using cargo make sure you got cargo install (https://doc.rust-lang.org/cargo/commands/cargo-install.html).
If you using docker make sure you docker daemon running [cargo/docker] ? "; 
    $conform=false;
    $input="";
    while($conform == false ){
        $input = trim(fgets(STDIN));
        $conform = ($input == "cargo"||$input == "docker");
        if($conform == false) {
            echo "('$input') is not a valid input you must choose between [cargo/docker] ? " ;
            $clearCount++;
        }
    }
    $codeOutput = ($input == "cargo")?2:1;
    echo str_repeat("\033[A\r\033[2K",$clearCount);

    exit($codeOutput);
}