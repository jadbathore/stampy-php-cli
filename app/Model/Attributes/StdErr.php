<?php

namespace Stampy\Model\Attributes;

use Attribute;

#[Attribute(self::TARGET_METHOD)]
class StdErr {
    public function __construct(
        private ?string $method=null,
    )
    {
        $this->method = $method;
    }
}