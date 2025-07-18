<?php

namespace App\Model\Class\Object;

use App\Model\Attributes\Command;
use App\Model\Attributes\Description;
use App\Model\Attributes\Option;
use App\Model\trait\Coloring;
use App\Model\Interface\MethodCLIInterface;
use App\Model\Class\Object\Option_CLI;
use \ReflectionMethod;

class Method_CLI implements MethodCLIInterface {

    use Coloring;

    private ?array $options;
    private array $promps = [];
    private ?string $description;
    private string $command;
    private object $invokable;

    public function __construct(private ReflectionMethod $method)
    {
        $this->setCommand();
        $this->setOptions();
        $this->setDescription();
        $this->setInvokable();
        $this->setPromps();
    }

    private function setOptions():void
    {
        $attributes_Options = $this->method->getAttributes(Option::class);
        foreach($attributes_Options as $attribute)
        {
            $this->addOptions($attribute->getArguments()[0]);
        }
    }
    private function setCommand():void
    {
        $attributes_Command = current($this->method->getAttributes(Command::class))->getArguments()[0];
        $this->command = $attributes_Command;
    }

    private function setDescription():void
    {
        $condition = (($a = current($this->method->getAttributes(Description::class))) != false);
        $attributes_Description = ($condition)?$a->getArguments()[0]:null;
        $this->description = $attributes_Description;
    }

    private function setInvokable():void
    {
        $className = $this->getClass();
        $this->invokable = new $className;
    }

    private function setPromps():void{
        foreach(array_keys($this->getOptions()??[]) as $option){
            $this->addPromps($option,null);
        }
        
    }

    public function getDescription(): ?string
    {
        return (isset($this->description))? $this->description:null;
    }

    /**
     * @return ?Object_CLI
     */
    public function getOptions(): null|array
    {
        return (isset($this->options))?$this->options[0]:null;
    }
    public function getCommand(): string
    {
        return $this->command;
    }

    private function addOptions(mixed $item):void
    {
        $this->options[] = $item;
    }

    public function addPromps(mixed $index,mixed $item):void
    {
        $this->promps[$index] = $item;
    }

    public function getPromps():null|array
    {
        return (isset($this->promps))?$this->promps:null; 
    }

    public function getName():string
    {
        return $this->method->name;
    }
    public function getClass():string
    {
        return $this->method->class;
    }

    public function invokeFromPromps(): void
    {
        if(is_null($this->getPromps()))
        {
            $this->method->invoke($this->invokable,$this->getPromps());
        } else {
            $this->method->invoke($this->invokable,...array_values($this->getPromps()));
        }
    }

    public function invoke(mixed ...$argument): void
    {
        $this->method->invoke($this->invokable,...$argument);
    }

    public function method_debug_script(string $color):void 
    {
        foreach($this->method->getAttributes() as $attribut)
        {
            $basename_attribut = $this->getBaseName($attribut->getName());
            $this->color($basename_attribut.":",$color,"underline","bold");
            switch($attribut->getName())
            {
                case Command::class:
                    $this->color($this->getCommand(),$color,"italic");
                break;
                case Option::class:
                    foreach ($this->getOptions() as $key => $value) {
                        if ($value instanceof Option_CLI){
                            $this->color("\n\t<$key>: {$value->getDescription()}",$color,"italic");
                        } else {
                        
                        }
                    }
                break;
                case Description::class:
                    $this->color($this->getDescription(),$color,"italic");
                break;
            }
            echo PHP_EOL;
        }
    }
    private function getBaseName(string $class):string
    {
        $explodeClass = explode("\\",$class);
        return $explodeClass[count($explodeClass)-1];
    }

    public function getLine():int|false
    {
        return $this->method->getStartLine();
    }

    public function getFile():string|false
    {
        return $this->method->getFileName();
    }
}