<?php

// $stream = json_decode(file_get_contents(getenv("COMPOSER")));

// if (!isset($stream?->scripts)){
//     $stream->scripts = new stdClass();
// }

// $stream->{"scripts"}->dockerStampy =  "./vendor/bin/dockerStampy";
// // $stream->{"scripts"}->{"dockerStampy"} = "./vendor/bin/dockerStampy";
// $decode = json_encode($stream, JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES);
// file_put_contents(getenv("COMPOSER"),$decode);



class ComposerHandler {
    private mixed $stream;

    public function __construct(string $json_file)
    {
        $this->stream = json_decode(file_get_contents($json_file));
    }

    public function get_arrayContent(){
        $autoloader = $this->stream->{"autoload"};
        return $autoloader?->{"psr-4"} ?? $autoloader?->{"psr-0"} ?? $autoloader?->{"classmap"} ?? $autoloader?->{"files"};
    }

    public function getlist(){
        $format = [];
        foreach((array) $this->get_arrayContent() as $key=>$value){
            $format[$key] = "namespace (\"$key\") for path (\"$value\")";
        }
        return $format; 
    }

    public function getSource(string $key){
        var_dump(((array) $this->get_arrayContent())[$key]);
        return ((array) $this->get_arrayContent())[$key];
    }

    public function add(string $key,string $value){
        $this->get_arrayContent()->{$key} = $value;
    }

    public function getNamespace(int $index){
        return array_keys($this->getlist())[$index];
    }
    
    public function getStream(){
        return $this->stream;
    }
}




$composer = new ComposerHandler(getenv("COMPOSER"));

$composer->add(getenv("NAMESPACE"),getenv("ENTRY"));
$decode = json_encode($composer->getStream(), JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES);
echo $decode;