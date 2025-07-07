<?php

namespace App\Model\Attributes;

use App\Model\Class\Object\Option_CLI;
use Attribute;

#[Attribute(self::TARGET_METHOD)]
class Option {

    /**
     * @param ?Option_CLI[] $method
     */
    public function __construct(
        private ?array $method=null,
    )
    {
        $this->method = $method;
    }
}