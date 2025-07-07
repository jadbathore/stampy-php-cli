<?php

namespace App\Model\Attributes;

use Attribute;

#[Attribute(self::TARGET_METHOD)]
class Description {
    public function __construct(
        private ?string $method=null,
    )
    {
        $this->method = $method;
    }
}