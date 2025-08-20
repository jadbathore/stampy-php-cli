<?php

namespace Stampy\Model\Class\Object;

use ReflectionAttribute;
use Stampy\Model\Attributes\Command;
use Stampy\Model\Attributes\Description;
use Stampy\Model\Attributes\Option;
use Stampy\Model\trait\Coloring;
use Stampy\Model\Interface\MethodCLIInterface;
use Stampy\Model\Class\Object\Option_CLI;
use \ReflectionMethod;
use Stampy\Model\Attributes\StdErr;
use Stampy\Model\Attributes\StdIn;
use Stampy\Model\Attributes\StdOut;

class Method_CLI implements MethodCLIInterface {

    use Coloring;

    private ?array $options;
    private array $promps = [];
    private ?string $description;
    private string $command;
    private object $invokable;
    private ?string $stdIn;
    private ?string $stdOut;
    private ?string $stdErr;

    public function __construct(private ReflectionMethod $method)
    {
        $this->setCommand();
        $this->setOptions();
        $this->setDescription();
        $this->setInvokable();
        $this->setPromps();
        $this->set_STDERR();
        $this->set_STDIN();
        $this->set_STDOUT();
    }

    public function useSTD(): bool
    {
        $s = $this->stdErr ?? $this->stdIn ?? $this->stdOut;
        return !is_null($s);
    }

    private function setOptions():void
    {
        $attributes_Options = $this->method->getAttributes(Option::class);
        foreach($attributes_Options as $attribute)
        {
            $this->addOptions($attribute->getArguments()[0]);
        }
    }

    private function set_STDIN():void{
        $this->stdIn = ($attrStdIn = current($this->method->getAttributes(StdIn::class)))? $attrStdIn->getArguments()[0] : null;
    }

    private function set_STDERR():void{
        $this->stdErr = ($attrStdErr = current($this->method->getAttributes(StdErr::class)))? $attrStdErr->getArguments()[0] : null;
    }

    private function set_STDOUT():void{
        $this->stdOut = ($attrStdOut = current($this->method->getAttributes(StdOut::class)))? $attrStdOut->getArguments()[0] : null;
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
        $args = (is_null($this->getPromps()))?
                $this->getPromps():
                array_values($this->getPromps());
        $this->method->invoke($this->invokable,...$args);
    }



    public function invoke(mixed ...$argument): void
    {
        $this->method->invoke($this->invokable,...$argument);
    }

    private function toDisplay(?string $attr):?string{
        switch($attr){
            case Command::class:
            case Option::class:
            case Description::class:
                return null;
            default: return "";
        }
    }  

    public function method_debug_script(string $color):void 
    {
        foreach($this->method->getAttributes() as $attribut)
        {
            $this->toDisplay($attribut->getName())??
            $this->color($this->getBaseName($attribut->getName()).":",$color,"underline","bold");
            switch($attribut->getName())
            {
                case Command::class:
                    $this->color($this->getCommand(),$color,"italic");
                break;
                case Option::class:
                    foreach ($this->getOptions() as $key => $value) {
                        if ($value instanceof Option_CLI){
                            $this->color("\n\t<$key>: {$value->getDescription()}",$color,"italic");
                        } 
                    }
                break;
                case Description::class:
                    $this->color($this->getDescription(),$color,"italic");
                break;
            }
            echo $this->toDisplay($attribut->getName()) ?? PHP_EOL;
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

    public function getStdErr():?string 
    {
        return $this->stdErr;
    }

    public function getStdIn():?string 
    {
        return $this->stdIn;
    }

    public function getStdOut():?string 
    {
        return $this->stdOut;
    }
}