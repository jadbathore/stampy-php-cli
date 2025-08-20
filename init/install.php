<?php

class ComposerHandler {
    private mixed $stream;

    public function __construct()
    {
        $this->stream = json_decode((null !== (getenv("COMPOSER")))?file_get_contents(getenv("COMPOSER")) :stream_get_contents(STDIN));
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
$use_ext= class_exists("Dialoguer");
$ques1 = "In witch namespace do you want to use your command-line-interface ?";
$ques2 = "Do you want to use";
$ques3 = "Do you want to do this later ?";

while ($confirm == false){
  
    $confirm1 = true;
    if (class_exists("Dialoguer")){
        $result = array_search(Dialoguer::select($ques1,$composer->getlist(),true),$composer->getlist());
        $confirm1 = Dialoguer::confirm("$ques2 $result ?",true);
        if($confirm1 == false){
            (Dialoguer::confirm("Do you want to do this later ?",true))?die():"";
        } 
    } else {
        $keys = array_keys($composer->getlist());
        echo $ques1 ." chose between ". "[". implode('|',array_keys($keys)) ."]";
        echo "\n> " . implode("\n> ",$keys);
        $result = trim(fgets(STDIN));
        $result_index = confirm($result,array_keys($keys));
        echo PHP_EOL;
        echo "$ques2". $composer->getlist()[$keys[$result_index]] ."? [y/n]";
        $result = trim(fgets(STDIN));
        $conf2 = confirm($result,['y','n']);
        $confirm1 = ($conf2 == "y");
        $result = $keys[$result_index];
        // echo $composer->getNamespace($result_index);
    }
    if($confirm1 == false){
        (Dialoguer::confirm("Do you want to do this later ?",true))?die():"";
    } else {
        $confirm = $confirm1;
    }
}

$arraykey = array_search($result,$composer->getlist());
$entry='ENTRY='.$composer->get_arrayContent()->{ $result }.'console/';
$namespace='NAMESPACE='.$result.'console\\controller';
$handle = fopen(getenv("STDOUT"), 'w');
fwrite($handle, "$entry\n$namespace");
fclose($handle);