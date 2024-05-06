# Name Corruptor

```
    agatha       => agadda       => agata        => agata        => agata       
    aldwin       => altwin       => altwen       => altwun       => altwum      
    althea       => alddea       => altea        => altea        => altea       
    anselm       => antzelm      => intzelm      => aintzelm     => aiwantzelm  
    armin        => armen        => armun        => armum        => armium      
    bartholomew  => partholomew  => fartholomew  => farddolomew  => fartolomew  
    berengar     => baerengar    => baerungar    => baerumgar    => baeriumgar  
    clarice      => clarihce     => claryce      => claryce      => claryce     
    constance    => konstance    => khonstance   => khonstince   => khonstaince 
    dierk        => dieyrk       => tieyrk       => tiheyrk      => tyeyrk      
    eadric       => eadrick      => eadrik       => eadrijk      => eatrijk     
    edward       => edvard       => etvard       => echard       => echart      
    eldrida      => eldrihda     => eldryda      => eltryta      => eltryta     
    elfric       => elfrihc      => elfryc       => elvryc       => elvhryc     
    erna         => erne         => aerne        => aerne        => aerne       
    eustace      => eustache     => eusteiche    => eusteeche    => eusteeckhe  
    felicity     => velicity     => vhelicity    => vhelihcihty  => vhelycyty   
    finnegan     => finnegin     => fainnegain   => vainnegain   => vhainnegain 
    giselle      => gisella      => gisellya     => gihsellya    => gysellya    
    gerald       => girald       => gidrald      => gitralt      => gihtralt    
    godric       => godrick      => godrik       => godrijk      => gotrijk     
    gunther      => gumther      => giumther     => gihumther    => gyumther    
    hadrian      => hadriin      => hadriain     => hadrayain    => hadrayaiwan 
    heloise      => heloihse     => heloyse      => heloize      => heloite     
    isolde       => isolte       => ihsolte      => ysolte       => izolte      
    ivor         => ivhor        => ihvhor       => yvhor        => yvhhor      
    jocelyn      => jocielyn     => joselyn      => jotzelyn     => jotzelyn    
    lancelot     => lancelod     => lancelot     => lincelot     => laincelot   
    lysandra     => lizandra     => litandra     => litzandra    => litsandra   
    magnus       => magnaes      => magnees      => magnees      => magnees     
    melisande    => melisante    => melihsante   => melysante    => melysinte   
    merrick      => merrich      => merrech      => merreckh     => maerreckh   
    osborn       => osporn       => osforn       => osvorn       => osvhorn     
    philomena    => philomene    => philomune    => philomume    => philomiume  
    reginald     => regihnald    => regynald     => regyneld     => regynelt    
    rowena       => rowuna       => rowuma       => rowiuma      => rowihuma    
    sabine       => sapine       => safine       => safaine      => zafaine     
    seraphina    => seraffina    => serafina     => seravina     => seravhina   
    sigfrid      => sigfrit      => sihgfriht    => sygfryt      => sygvryt     
    tiberius     => tibaerius    => tihbaerihus  => tybaeryus    => typaeryus   
    ulf          => ulv          => ulvh         => ulvh         => ulvh        
    urien        => uriun        => urium        => uriium       => urihihum    
    vespera      => vesfera      => vesfira      => vesfidra     => vesvidra    
    wendel       => wentel       => wuntel       => wumtel       => wiumtel     
    wilfred      => wihlfred     => wylfred      => wylvred      => wylvhred    
    winifred     => winifret     => wenifret     => wunifret     => wumifret    
    xenia        => xeniha       => xenya        => xunya        => xumya       
    ysabel       => ysapel       => ysafel       => izafel       => itafel      
    zephyr       => zeffyr       => zefyr        => zevyr        => zevhyr      
    zinnia       => zennia       => zunnia       => zumnia       => ziumnia     
    zuriel       => zurihel      => zuryel       => tzuryel      => tzuryel     
    zygmund      => zygmunt      => zygmumt      => zygmiumt     => zygmihumt   
```

This implements a little idea I was thinking about around generating fantasy names from historical ones --

So I was thinking that I like fantasy names that are variants of more historic names - think Neddard Stark from Edward of York, Geralt from Gerald, that sort of thing. So I was wondering how you might generate that. 

There's a bit of linguistics called Grimm's Law, that shows how European languages shift certain consonants about. Eg, a 'd' in german can often become a 't' or a 'th' in English, and a 'p' in french becomes an 'f' in english.

here's the laws;

* bʰ → b → p → f
* dʰ → d → t → θ
* gʰ → g → k → x
* gʷʰ → gʷ → kʷ → xʷ

and here's some examples;

broder -> brother, 
pied -> foot
dent -> tooth
grain -> corn

So I'm thinking this is the sort of thing you could use for "gerald -> geralt" because d->t.

So a rough idea:

- split a word into characters or maybe little sounds like 'er' -- eg { G ER AL D }
- search for shifts you can make according to rules - eg { G ER  AL T < D } 
- reconstitute it into a new word - GERALT

Then you can generate names by starting with a list of common names, and applying one or more 'shift operations' on it, and see what pops out. 

So maybe something like 

    Edward -> Edard -> Nedard -> Nethart

Might help with generating names that 'feel a bit right' -- so probably for games that have a European feel, like Warhammer FRP, or the Witcher, etc. 
