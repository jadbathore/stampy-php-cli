<?php

namespace Stampy\Model\Enum;

use Stampy\Model\Interface\ArgvInterface;
use Stampy\Model\Interface\MethodCLIInterface;
use Stampy\Model\class\Object\Option_CLI;

enum Argv:string 
{
    case Option="option";
    case Command="command";
    case Input="input";
    case Empty="empty";
    case unknownInput="unknownInput";
    
    
    public static function type_Argv(ArgvInterface $argv,MethodCLIInterface $methodCLI):self
    {
        $optionList = $methodCLI->getOptions() ?? [];
        
        return match(true)
        {
            (in_array($argv->getCurrent(),array_keys($optionList))) => static::Option,
            (self::inputTypeHandler($optionList,$argv)) => static::Input,
            (is_null($argv->getCurrent()))=> static::Empty,
            ($methodCLI->getCommand() == $argv->getCurrent()) => static::Command,
            default=>static::unknownInput
        };
    }
    
    /**
     * @param Option_CLI[] $optionList
     */
    private static function inputTypeHandler(array $optionList,ArgvInterface $argv):bool {
        if(in_array($argv->getLast(),array_keys($optionList))){
            return $optionList[$argv->getLast()]->getAdInput();
        }
        return false;
    }
}