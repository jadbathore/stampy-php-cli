<?php

namespace App\Model\Class\Object;

use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use App\Model\trait\Coloring;
use App\Model\Interface\MethodCLIInterface;
use \ReflectionMethod;


class Option_CLI {
    public function __construct(
        private bool $ad_input,
        private ?string $description = null
    ){
    }

    public function getAdInput():bool
    {
        return $this->ad_input;
    }

    public function getDescription():?string
    {
        return $this->description;
    }
}

