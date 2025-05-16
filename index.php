<?php 

require_once "vendor/autoload.php";

class test {
    public $dialoguer =  Dialoguer::class;

    public function test(){
        $this->dialoguer::confirm("bonjour");
    }
}
new test()->test();
// Dialoguer::select('choisis parmit c\'est reponse ',["a","b","c"]);
// $test = Dialoguer::multiSelect('choisis parmit c\'est reponse ',["a","b","c"]);
// var_dump($test);
// $test = Dialoguer::confirm("bonjour est que ça va ?");

// if ($test){
//     echo "super";
// } else {
//     echo "ça va aller t'inquete";
// }

// $name = Dialoguer::input("comment tu t'appel");

// echo "salut $name";