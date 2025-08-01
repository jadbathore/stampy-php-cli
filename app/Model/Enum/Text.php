<?php

namespace Stampy\Model\Enum;

use Error;
use Stampy\Model\Interface\ColorInterface;

enum Text:string implements ColorInterface
{
    case TextModif="TextModif";
    case Coloring="Coloring";
    case bgColoring="BgColoring";
    case Error="Error";

    private static function type_text(string $input):self
    {
        return match(true)
        {
            (str_contains($input,"bg"))=> static::bgColoring,
            (in_array($input,self::Colour))=>static::Coloring,
            (in_array($input,self::acceptableTextModif))=>static::TextModif,
            default=>static::Error,
        };
    }

    static public function formatColoring(string $input,mixed ...$textmodif):string
    {
        $getType = self::type_text($input);
        $return_value = match ($getType) {
            self::bgColoring => "4",
            self::Coloring => "3", 
            self::Error=>"0",
            default=> throw new Error("type no allowed.for input($getType)")
        };
        if($getType == self::bgColoring)
        {
            $input = implode(explode('bg',$input));
        }
        $textmodifArray = [];
        foreach(current($textmodif) as $argument_modif)
        {
            $textmodifArray[] = Text::formatTextModif($argument_modif ?? '');
        }
        $key = array_search($input,self::Colour);
        $textmodification =(empty($textmodif))?'':implode('',$textmodifArray);
        return "\e[".$return_value.$key.$textmodification."m";
    }

    static private function formatTextModif(string $input):?string
    {
        if(self::type_text($input) == self::TextModif)
        {
            $key = array_search($input,self::acceptableTextModif);
            return ";".$key;
        } else {
            return null;
        }
    }
}