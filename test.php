<?php 

class Option_CLI {
    public function __construct(
        private bool $ad_input,
        private ?string $description = null
    ){    }

    public function getAdInput(){
        return $this->ad_input;
    }

    public function getDescription(){
        return $this->description;
    }
}

$a = new Option_CLI(false,"bla blba");
$b = new Option_CLI(false,"bla blba");


/**
 * @var Array<Option_CLI>
 */
$test = [
    "a" => (new Option_CLI(false,"bla blba")),
    "b" => (new Option_CLI(true,"bla blba"))
];

var_dump($test["a"]->getAdInput());
var_dump($test["b"]->getDescription());