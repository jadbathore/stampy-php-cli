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
    echo "the stampy extension add no pré-compile binairy for your architecture you can  ?
    you could compile by yourself using cargo or use docker
    if you using cargo make sure you got cargo install (https://doc.rust-lang.org/cargo/commands/cargo-install.html)
    if you using docker make sure you docker damion running
    [cargo/docker]\n
    "; 
    $conform=false;
    $input="";
    while($conform == false ){
        $input = trim(fgets(STDIN));
        $conform = ($input == "cargo"||$input == "docker");
        if($conform == false) {
            echo "('$input') is not a valid input you must choose between [cargo/docker]\n" ;
        }
    }
    $codeOutput = ($input == "cargo")?2:1;
    exit($codeOutput);
}