<?php

namespace App\Model\Attributes;

use Attribute;

#[Attribute]
class Description {
    public function __construct(
        private ?string $method=null,
    )
    {
        $this->method = $method;
    }
}