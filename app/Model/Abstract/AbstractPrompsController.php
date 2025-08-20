<?php

namespace Stampy\Model\Abstract;

use ArrayObject;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Class\Object\Option_CLI;
use Stampy\Model\Class\SingleTone\Organisator;
use Stampy\Model\Trait\Coloring;
use Indicatif;


abstract class abstractPrompsController
{
    use Coloring;

    public function newProgressBar(int $length)
    {
        return new Indicatif($length);
    }

    public function newOption(bool $input,?string $description = null){
        return new Option_CLI($input,$description);
    }
    
    
} 