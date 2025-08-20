<?php

namespace Stampy\Model\Attributes;

use Attribute;

#[Attribute(self::TARGET_METHOD)]
class StdIn {
    public function __construct(
        private ?string $method=null,
    )
    {
        $this->method = $method;
    }
}