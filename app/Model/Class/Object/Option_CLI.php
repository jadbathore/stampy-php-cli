<?php

namespace App\Model\Class\Object;


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

