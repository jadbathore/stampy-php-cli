<?php

namespace Stampy\Model\Attributes;

use Attribute;

#[Attribute(self::TARGET_METHOD)]
class StdOut {
    public function __construct(
        private ?string $method=null,
    )
    {
        $this->method = $method;
    }
}