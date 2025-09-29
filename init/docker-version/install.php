<?php

class ComposerHandler {
    private mixed $stream;

    public function __construct()
    {
        $this->stream = json_decode(file_get_contents(getenv("COMPOSER")));
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

function confirm(string $result,array $keys) {
    $not_correct = !in_array($result,$keys);
    while ($not_correct){
        echo "must be [".implode("|",$keys)."]";
        $result = trim(fgets(STDIN));
        $not_correct = !in_array($result,$keys);
    }
    return $result;
}

$confirm = false;
$key;$value;
$composer = new ComposerHandler();
$ques1 = "In witch namespace do you want to use your command-line-interface ?";
$ques2 = "Do you want to use";
$ques3 = "Do you want to do this later ?";

if (count($composer->getlist()) >= 2){
    while (!$confirm){
        $key = array_search(Dialoguer::select($ques1,$composer->getlist(),true),$composer->getlist());
        $confirm = Dialoguer::confirm("$ques2 $key ?",true);
    }   
    $value = $composer->get_arrayContent()->{ $key };
} else {
    $value = array_values((array) $composer->get_arrayContent())[0];
    $key = array_keys((array) $composer->get_arrayContent())[0];
}

$stampy = "StampyConsole";
echo "NAMESPACE=$stampy";
echo PHP_EOL;
echo 'ENTRY='.$value."console/";
$composer->add("$stampy\\",$value."console/");
$decode = json_encode($composer->getStream(), JSON_PRETTY_PRINT | JSON_UNESCAPED_SLASHES);
file_put_contents(getenv("COMPOSER"),$decode);