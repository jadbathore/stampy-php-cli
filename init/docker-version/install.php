<?php

class ComposerHandler {
    private mixed $stream;

    public function __construct()
    {
        $this->stream = json_decode((null !== (getenv("COMPOSER")))?file_get_contents(getenv("COMPOSER")) :stream_get_contents(STDIN));
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

    public function getNamespace(int $index){
        return array_keys($this->getlist())[$index];
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
$result;
$composer = new ComposerHandler();
$ques1 = "In witch namespace do you want to use your command-line-interface ?";
$ques2 = "Do you want to use";
$ques3 = "Do you want to do this later ?";
if (count($composer->getlist()) >= 2){
    while ($confirm == false){
        $result = array_search(Dialoguer::select($ques1,$composer->getlist(),true),$composer->getlist());
        $confirm = Dialoguer::confirm("$ques2 $result ?",true);
    }   
    $arraykey = array_search($result,$composer->getlist());
    echo 'ENTRY='.$composer->get_arrayContent()->{ $result }.'console/';
    echo PHP_EOL;

    echo 'NAMESPACE='.$result.'console\\controller';
} else {
    echo 'ENTRY='. array_values((array) $composer->get_arrayContent())[0] .'console/';
    echo PHP_EOL;
    echo 'NAMESPACE='. array_keys((array) $composer->get_arrayContent())[0] .'console\\controller';
}
