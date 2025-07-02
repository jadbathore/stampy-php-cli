<?php

namespace App\Model\Class\Object;

use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use App\Model\trait\Coloring;
use App\Model\Interface\MethodCLIInterface;
use \ReflectionMethod;

class Option_CLI {

    public private(set) bool $ad_input;

    public private(set) ?string $description;

    public function __construct(
        bool $ad_input,
        ?string $description = null
    ){
        $this->$ad_input = $ad_input;
        $this->$description = $description;
    }
}

