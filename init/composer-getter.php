<?php

class ComposerHandler {
    private mixed $stream;

    public function __construct()
    {
        $this->stream = json_decode(stream_get_contents(STDIN));
    }

    public function get_arrayContent(){
        $autoloader = $this->stream->{"autoload"};
        return $autoloader?->{"psr-4"}??$autoloader?->{"psr-0"}??$autoloader?->{"classmap"}??$autoloader?->{"files"};
    }

    public function getlist(){
        $format = [];
        foreach((array) $this->get_arrayContent() as $key=>$value){
            $format[$key] = "namespace (\"$key\") for path (\"$value\")";
        }
        return $format; 
    }
}

$composer= new ComposerHandler();
echo "AUTOLOADERCONTENT=".implode(",",$composer->getlist());

// function composergetter(){
//     $stream = json_decode(stream_get_contents(STDIN));
//     $autoloader = $stream->{"autoload"};
//     $arrayContent =  $autoloader?->{"psr-4"}??$autoloader?->{"psr-0"}??$autoloader?->{"classmap"}??$autoloader?->{"files"};
//     $format = [];

//     foreach((array) $arrayContent as $key=>$value){
//         $format[$key] = "namespace (\"$key\") for path (\"$value\")";
//     }

//     return $format;
// }