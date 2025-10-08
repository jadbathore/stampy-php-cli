<?php
echo PHP_EOL;
$select = ["continue as such","use docker"];
$input = Dialoguer::select("the stampy extension add already a pré-compile binairy you can  ? ",
$select,true);

if ($input == $select[1]){
    exit(1);
}

// if (php_sapi_name() !== 'cli') {
//     exit(130);
// }