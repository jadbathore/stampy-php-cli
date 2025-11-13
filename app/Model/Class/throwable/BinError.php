<?php

namespace Stampy\Model\Class\Throwable;

use Stampy\Model\Class\Object\Method_CLI;
use Stampy\Model\Enum\Error;
use Stampy\Model\Interface\MethodCLIInterface;
use Exception;
use ReflectionMethod;
use Throwable;

class BinError extends Exception {

    public private(set) Error $errorType;
    public private(set) string $className;
    public private(set) ?Method_CLI $method1;
    public private(set) ?Method_CLI $method2;

    public function __construct(
        Error $errorType,
        ?string $className,
        ?ReflectionMethod $method1 = null,
        ?MethodCLIInterface $method2 = null,
        int $code = 0 ,
        ?Throwable $throwable = null)
    {
        parent::__construct("",$code,$throwable);
        $this->errorType = $errorType;
        $this->className = $className;
        // $this->method1 ;
        $this->method1 = (isset($method1))? new Method_CLI($method1) : null;
        $this->method2 = $method2;
    }
}