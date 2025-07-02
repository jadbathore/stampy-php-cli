<?php

namespace App\Model\Attributes;

use App\Model\Class\Object\Option_CLI;
use Attribute;

#[Attribute]
class Option {
    /**
     * @param Option_CLI[]
     */
    public function __construct(
        private ?array $method=null,
    )
    {
        $this->method = $method;
    }
}