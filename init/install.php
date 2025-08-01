<?php

$stream = json_decode(stream_get_contents(STDIN));

$autoloader = $stream->{"autoload"};
$arrayContent =  $autoloader?->{"psr-4"}??$autoloader?->{"psr-0"}??$autoloader?->{"classmap"}??$autoloader?->{"files"};
$format = [];

foreach((array) $arrayContent as $key=>$value){
    $format[$key] = "namespace (\"$key\") for path (\"$value\")";
}

$confirm = false;
$result;
while ($confirm == false){
    $result = Dialoguer::select("In witch namespace do you want to use your command-line-interface ?",$format,true);
    $confirm1 = Dialoguer::confirm("Do you want to use $result",true);
    if($confirm1 == false){
        (Dialoguer::confirm("Do you want to do this later ?",true))?die():"";
    } else {
        $confirm = $confirm1;
    }
}

echo 'ENTRY="'.$arrayContent->{array_search($result,$format)}.'"';
echo PHP_EOL;
echo 'NAMESPACE="'.array_search($result,$format).'"';