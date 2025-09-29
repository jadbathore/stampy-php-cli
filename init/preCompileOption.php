<?php
echo PHP_EOL;
$select = ["continue as such","use docker"];
$input = Dialoguer::select("the stampy extension add already a pré-compile binairy you can  ? ",
$select,true);
if ($input == $select[1]){
    exit(1);
}

if (php_sapi_name() !== 'cli') {
    exit(130);
}

// if (class_exists("Dialoguer")) {
//     echo PHP_EOL;
//     $select = ["continue as such","use docker"];
//     $input = Dialoguer::select("the stampy extension add already a pré-compile binairy you can  ? ",
//     $select,true);
//     if ($input == $select[1]){
//         exit(1);
//     }
// } else {
// $clearCount=3;

// // $installer = new Installer();
// // $projectName = $installer->io->ask("Entrez le nom du projet : ");
// echo "the stampy extension add no pré-compile binairy for your architecture you can compile the binairy by yourself \n
// using cargo or use docker.If you using cargo make sure you got cargo install (https://doc.rust-lang.org/cargo/commands/cargo-install.html). \n
// If you using docker make sure you docker daemon running [cargo/docker] ? ";
//     $conform=false;
//     $input="";
//     while($conform == false){
//         $input = trim(fgets(STDIN));
//         $conform = ($input == "cargo"||$input == "docker");
//         if($conform == false) {
//             echo "('$input') is not a valid input you must choose between [cargo/docker] ? " ;
//             $clearCount++;
//         }
//     }
//     $codeOutput = ($input == "cargo")?2:1;
//     echo str_repeat("\033[A\r\033[2K",$clearCount);

//     exit($codeOutput);
// }