use crate::colors;
use crate::fetch::distro_id;

struct Logo {
    logo: &'static str,
    main_color: usize,
}

const LOGOS: &[(&str, Logo)] = &[
    (
        "arch",
        Logo {
            logo: r#"{4}
                   -`
                  .o+`
                 `ooo/,
                `+oooo:,
               `+oooooo:,
              , `!-oooo+:,
             `/+;.:-oooo+:,
            `/+++-;+++++++:,
           `/+++++++++++ooo:.
          `/+++ooooooooooooo/`
        ./ooosssso++ossssss-:+`
        .oossssso-````-oss-!,`+'
       -osssssso.      :ssss:;.,
      :ossssssss/        osssso-++.
     /ossssssss/        +ssssooo/.
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+:
 \.`                               ``/
"#, main_color: 4, },
    ),

    (
        "nixos",
        Logo {
            logo: r#"
{4}          ▗▄▄▄       {6}▗▄▄▄▄    ▄▄▄▖{4}
{4}          ▜███▙       {6}▜███▙  ▟███▛{4}
{4}           ▜███▙       {6}▜███▙▟███▛{4}
{4}            ▜███▙       {6}▜██████▛{4}
{4}     ▟█████████████████▙ {6}▜████▛     {4}▟▙
{4}    ▟███████████████████▙ {6}▜███▙    {4}▟██▙
{6}           ▄▄▄▄▖           ▜███▙  {4}▟███▛
{6}          ▟███▛             ▜██▛ {4}▟███▛
{6}         ▟███▛               ▜▛ {4}▟███▛
{6}▟███████████▛                  {4}▟██████████▙
{6}▜██████████▛                  {4}▟███████████▛
{6}      ▟███▛ {4}▟▙               ▟███▛
{6}     ▟███▛ {4}▟██▙             ▟███▛
{6}    ▟███▛  {4}▜███▙           ▝▀▀▀▀
{6}    ▜██▛    {4}▜███▙ {6}▜██████████████████▛{4}
{6}     ▜▛     {4}▟████▙ {6}▜████████████████▛{4}
{4}           ▟██████▙       {6}▜███▙{4}
{4}          ▟███▛▜███▙       {6}▜███▙{4}
{4}         ▟███▛  ▜███▙       {6}▜███▙{4}
{4}         ▝▀▀▀    ▀▀▀▀▘       {6}▀▀▀▘{4}
"#, main_color: 4, },
    ),
    
    (
        "debian",
        Logo {
            logo: r#"
        _,met$$$$$gg.
    ,g$$$$$$$$$$$$$$$P.
  ,g$$P"     """Y$$.".
 ,$$P'              `$$$.
',$$P       ,ggs.     `$$b:
`d$$'     ,$P"'   {1}.{7}    $$$
 $$P      d$'     {1},{7}    $$P
 $$:      $$.   {1}-{7}    ,d$$'
 $$;      Y$b._   _,d$P'
 Y$$.    {1}.{7}`"Y$$$$P"'
{7} `$$b      {1}"-.__
{7}  `Y$$
   `Y$$.
     `$$b.
       `Y$$b.
          `"Y$b._
              `"""`
"#, main_color: 7, },
    ),

    (
        "ubuntu",
        Logo {
            logo: r#"
            .-/+oossssoo+/-.
        `:+ssssssssssssssssss+:`
      -+ssssssssssssssssssyyssss+-
    .ossssssssssssssssss{7}dMMMNy{1}sssso.
   /sssssssssss{7}hdmmNNmmyNMMMMh{1}ssssss/
  +sssssssss{7}hm{1}yd{7}MMMMMMMNddddy{1}ssssssss+
 /ssssssss{7}hNMMM{1}yh{7}hyyyyhmNMMMNh{1}ssssssss/
.ssssssss{7}dMMMNh{1}ssssssssss{7}hNMMMd{1}ssssssss.
+ssss{7}hhhyNMMNy{1}ssssssssssss{7}yNMMMy{1}sssssss+
oss{7}yNMMMNyMMh{1}ssssssssssssss{7}hmmmh{1}ssssssso
oss{7}yNMMMNyMMh{1}sssssssssssssshmmmh{1}ssssssso
+ssss{7}hhhyNMMNy{1}ssssssssssss{7}yNMMMy{1}sssssss+
.ssssssss{7}dMMMNh{1}ssssssssss{7}hNMMMd{1}ssssssss.
 /ssssssss{7}hNMMM{1}yh{7}hyyyyhdNMMMNh{1}ssssssss/
  +sssssssss{7}dm{1}yd{7}MMMMMMMMddddy{1}ssssssss+
   /sssssssssss{7}hdmNNNNmyNMMMMh{1}ssssss/
    .ossssssssssssssssss{7}dMMMNy{1}sssso.
      -+sssssssssssssssss{7}yyy{1}ssss+-
        `:+ssssssssssssssssss+:`
            .-/+oossssoo+/-.
"#, main_color: 1, },
    ),

    (
        "fedora",
        Logo {
            logo: r#"
         /:-------------:\
       :-------------------::
     :-----------{7}/shhOHbmp{4}---:\
   /-----------{7}omMMMNNNMMD  {4}---:
  :-----------{7}sMMMMNMNMP{4}.    ---:
 :-----------{7}:MMMdP{4}-------    ---\
,------------{7}:MMMd{4}--------    ---:
:------------{7}:MMMd{4}-------    .---:
:----    {7}oNMMMMMMMMMNho{4}     .----:
:--     .{7}+shhhMMMmhhy++{4}   .------/
:-    -------{7}:MMMd{4}--------------:
:-   --------{7}/MMMd{4}-------------;
:-    ------{7}/hMMMy{4}------------:
:--{7} :dMNdhhdNMMNo{4}------------;
:---{7}:sdNMMMMNds:{4}------------:
:------{7}:://:{4}-------------::
:---------------------:/
"#, main_color: 4, },
    ),

   (
        "centos",
        Logo {
            logo: r#"
                 ..
               .PLTJ.
              <><><><>
     {2}KKSSV' 4KKK {3}LJ{5} KKKL.'VSSKK
     {2}KKV' 4KKKKK {3}LJ{5} KKKKAL 'VKK
     {2}V' ' 'VKKKK {3}LJ{5} KKKKV' ' 'V
     {2}.4MA.' 'VKK {3}LJ{5} KKV' '.4Mb.
{5}   . {2}KKKKKA.' 'V {3}LJ{5} V' '.4KKKKK {4}.
{5} .4D {2}KKKKKKKA.'' {3}LJ{5} ''.4KKKKKKK {4}FA.
{5}<QDD ++++++++++++  {4}++++++++++++ GFD>
{5} 'VD {4}KKKKKKKK'.. {2}LJ {3}..'KKKKKKKK {4}FV
{5}   ' {4}VKKKKK'. .4 {2}LJ {3}K. .'KKKKKV {4}'
     {4} 'VK'. .4KK {2}LJ {3}KKA. .'KV'
     {4}A. . .4KKKK {2}LJ {3}KKKKA. . .4
     {4}KKA. 'KKKKK {2}LJ {3}KKKKK' .4KK
     {4}KKSSA. VKKK {2}LJ {3}KKKV .4SSKK
{2}              <><><><>
               'MKKM'
                 ''
"#, main_color: 4, },
    ), 

    (
        "manjaro",
        Logo {
            logo: r#"
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
████████            ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
"#, main_color: 2, },
    ),

    (
        "alpine",
        Logo {
            logo: r#"
       .hddddddddddddddddddddddh.
      :dddddddddddddddddddddddddd:
     /dddddddddddddddddddddddddddd/
    +dddddddddddddddddddddddddddddd+
  `sdddddddddddddddddddddddddddddddds`
 `ydddddddddddd++hdddddddddddddddddddy`
.hddddddddddd+`  `+ddddh:-sdddddddddddh.
hdddddddddd+`      `+y:    .sddddddddddh
ddddddddh+`   `//`   `.`     -sddddddddd
ddddddh+`   `/hddh/`   `:s-    -sddddddd
ddddh+`   `/+/dddddh/`   `+s-    -sddddd
ddd+`   `/o` :dddddddh/`   `oy-    .yddd
hdddyo+ohddyosdddddddddho+oydddy++ohdddh
.hddddddddddddddddddddddddddddddddddddh.
 `yddddddddddddddddddddddddddddddddddy`
  `sdddddddddddddddddddddddddddddddds`
    +dddddddddddddddddddddddddddddd+
     /dddddddddddddddddddddddddddd/
      :dddddddddddddddddddddddddd:
       .hddddddddddddddddddddddh.
"#, main_color: 4, },
    ),

    (
        "linuxmint",
        Logo {
            logo: r#"
             ...-:::::-...
{15}          .-MMMMMMMMMMMMMMM-.
      .-MMMM{10}`..-:::::::-..`{15}MMMM-.
    .:MMMM{10}.:MMMMMMMMMMMMMMM:.{15}MMMM:.
   -MMM{10}-M---MMMMMMMMMMMMMMMMMMM.{15}MMM-
 `:MMM{10}:MM`  :MMMM:....::-...-MMMM:{15}MMM:`
 :MMM{10}:MMM`  :MM:`  ``    ``  `:MMM:{15}MMM:
.MMM{10}.MMMM`  :MM.  -MM.  .MM-  `MMMM.{15}MMM.
:MMM{10}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{15}MMM:
:MMM{10}:MMMM`  :MM.  -MM-  .MM:  `MMMM:{15}MMM:
:MMM{10}:MMMM`  :MM.  -MM-  .MM:  `MMMM-{15}MMM:
.MMM{10}.MMMM`  :MM:--:MM:--:MM:  `MMMM.{15}MMM.
 :MMM{10}:MMM-  `-MMMMMMMMMMMM-`  -MMM-{15}MMM:
  :MMM{10}:MMM:`                `:MMM:{15}MMM:
   .MMM{10}.MMMM:--------------:MMMM.{15}MMM.
     '-MMMM{10}.-MMMMMMMMMMMMMMM-.{15}MMMM-'
       '.-MMMM{10}``--:::::--``{15}MMMM-.'
{15}            '-MMMMMMMMMMMMM-'
{15}               ``-:::::-``
"#, main_color: 15, },
    ),

    (
        "opensuse-leap",
        Logo {
            logo: r#"
                 `-++:`
               ./oooooo/-
            `:oooooooooooo:.
          -+oooooooooooooooo+-`
       ./oooooooooooooooooooooo/-
      :oooooooooooooooooooooooooo:
    `  `-+oooooooooooooooooooo/-   `
 `:oo/-   .:ooooooooooooooo+:`  `-+oo/.
`/oooooo:.   -/oooooooooo/.   ./oooooo/.
  `:+ooooo+-`  `:+oooo+-   `:oooooo+:`
     .:oooooo/.   .::`   -+oooooo/.
        -/oooooo:.    ./oooooo+-
          `:+ooooo+-:+oooooo:`
             ./oooooooooo/.
                -/oooo+:`
                  `:/.
"#, main_color: 7, },
    ),

    (
        "kali",
        Logo {
            logo: r#"
..............
            ..,;:ccc,.
          ......''';lxO.
.....''''..........,:ld;
           .';;;:::;,,.x,
      ..'''.            0Xxoc:,.  ...
  ....                ,ONkc;,;cokOdc',.
 .                   OMo           ':${4}dd{7}o.
                    dMc               :OO;
                    0M.                 .:o.
                    ;Wd
                     ;XO,
                       ,d0Odlc;,..
                           ..',;:cdOOd::,.
                                    .:d;.':;.
                                       'd,  .'
                                         ;l   ..
                                          .o
                                            c
                                            .'
                                             .
"#, main_color: 4, },
    ),

    (
        "parrot",
        Logo {
            logo: r#"
  `:oho/-`
`mMMMMMMMMMMMNmmdhy-
 dMMMMMMMMMMMMMMMMMMs`
 +MMsohNMMMMMMMMMMMMMm/
 .My   .+dMMMMMMMMMMMMMh.
  +       :NMMMMMMMMMMMMNo
           `yMMMMMMMMMMMMMm:
             /NMMMMMMMMMMMMMy`
              .hMMMMMMMMMMMMMN+
                  ``-NMMMMMMMMMd-
                     /MMMMMMMMMMMs`
                      mMMMMMMMsyNMN/
                      +MMMMMMMo  :sNh.
                      `NMMMMMMm     -o/
                       oMMMMMMM.
                       `NMMMMMM+
                        +MMd/NMh
                         mMm -mN`
                         /MM  `h:
                          dM`   .
                          :M-
                           d:
                           -+
                            -
"#, main_color: 6, },
    ),

    (
        "zorin",
        Logo {
            logo: r#"
         `osssssssssssssssssssso`
       .osssssssssssssssssssssso.
      .+oooooooooooooooooooooooo+.


  `::::::::::::::::::::::.         .:`
 `+ssssssssssssssssss+:.`     `.:+ssso`
.ossssssssssssssso/.       `-+ossssssso.
ssssssssssssso/-`      `-/osssssssssssss
.ossssssso/-`      .-/ossssssssssssssso.
 `+sss+:.      `.:+ssssssssssssssssss+`
  `:.         .::::::::::::::::::::::`


      .+oooooooooooooooooooooooo+.
       -osssssssssssssssssssssso-
        `osssssssssssssssssssso`
"#, main_color: 4, },
    ),

    (
        "slackware",
        Logo {
            logo: r#"
                  :::::::
            :::::::::::::::::::
         :::::::::::::::::::::::::
       ::::::::{7}cllcccccllllllll{4}::::::
    :::::::::{7}lc               dc{4}:::::::
   ::::::::{7}cl   clllccllll    oc{4}:::::::::
  :::::::::{7}o   lc{4}::::::::{7}co   oc{4}::::::::::
 ::::::::::{7}o    cccclc{4}:::::{7}clcc{4}::::::::::::
 :::::::::::{7}lc        cclccclc{4}:::::::::::::
::::::::::::::{7}lcclcc          lc{4}::::::::::::
::::::::::{7}cclcc{4}:::::{7}lccclc     oc{4}:::::::::::
::::::::::{7}o    l{4}::::::::::{7}l    lc{4}:::::::::::
 :::::{7}cll{4}:{7}o     clcllcccll     o{4}:::::::::::
 :::::{7}occ{4}:{7}o                  clc{4}:::::::::::
  ::::{7}ocl{4}:{7}ccslclccclclccclclc{4}:::::::::::::
   :::{7}oclcccccccccccccllllllllllllll{4}:::::
    ::{7}lcc1lcccccccccccccccccccccccco{4}::::
      ::::::::::::::::::::::::::::::::
        ::::::::::::::::::::::::::::
           ::::::::::::::::::::::
                ::::::::::::
"#, main_color: 4, },
    ),

    (
        "solus",
        Logo {
            logo: r#"
            -```````````
          `-+/------------.`
       .---:mNo---------------.
     .-----yMMMy:---------------.
   `------oMMMMMm/----------------`
  .------/MMMMMMMN+----------------.
 .------/NMMMMMMMMm-+/--------------.
`------/NMMMMMMMMMN-:mh/-------------`
.-----/NMMMMMMMMMMM:-+MMd//oso/:-----.
-----/NMMMMMMMMMMMM+--mMMMh::smMmyo:--
----+NMMMMMMMMMMMMMo--yMMMMNo-:yMMMMd/.
.--oMMMMMMMMMMMMMMMy--yMMMMMMh:-yMMMy-`
`-sMMMMMMMMMMMMMMMMh--dMMMMMMMd:/Ny+y.
`-/+osyhhdmmNNMMMMMm-/MMMMMMMmh+/ohm+
  .------------:://+-/++++++{4}oshddys:
   -hhhhyyyyyyyyyyyhhhhddddhysssso-
    `:ossssssyysssssssssssssssso:`
      `:+ssssssssssssssssssss+-
         `-/+ssssssssssso+/-`
              `.-----..`
"#, main_color: 7, },
    ),

    (
        "puppy",
        Logo {
            logo: r#"
           `-/osyyyysosyhhhhhyys+-
  -ohmNNmh+/hMMMMMMMMNNNNd+dMMMMNM+
 yMMMMNNmmddo/NMMMNNNNNNNNNo+NNNNNy
.NNNNNNmmmddds:MMNNNNNNNNNNNh:mNNN/
-NNNdyyyhdmmmd`dNNNNNmmmmNNmdd/os/
.Nm+shddyooo+/smNNNNmmmmNh.   :mmd.
 NNNNy:`   ./hmmmmmmmNNNN:     hNMh
 NMN-    -++- +NNNNNNNNNNm+..-sMMMM-
.MMo    oNNNNo hNNNNNNNNmhdNNNMMMMM+
.MMs    /NNNN/ dNmhs+:-`  yMMMMMMMM+
 mMM+     .. `sNN+.      hMMMMhhMMM-
 +MMMmo:...:sNMMMMMms:` hMMMMm.hMMy
  yMMMMMMMMMMMNdMMMMMM::/+o+//dMMd`
   sMMMMMMMMMMN+:oyyo:sMMMNNMMMNy`
    :mMMMMMMMMMMMmddNMMMMMMMMmh/
      /dMMMMMMMMMMMMMMMMMMNdy/`
        .+hNMMMMMMMMMNmdhs/.
            .:/+ooo+/:-.
"#, main_color: 4, },
    ),

    (
        "tails",
        Logo {
            logo: r#"
     ``
  ./yhNh
syy/Nshh         `:o/
N:dsNshh  █   `ohNMMd
N-/+Nshh      `yMMMMd
N-yhMshh       yMMMMd
N-s:hshh  █    yMMMMd so//.
N-oyNsyh       yMMMMd d  Mms.
N:hohhhd:.     yMMMMd  syMMM+
Nsyh+-..+y+-   yMMMMd   :mMM+
+hy-      -ss/`yMMMM     `+d+
  :sy/.     ./yNMMMMm      ``
    .+ys- `:+hNMMMMMMy/`
      `hNmmMMMMMMMMMMMMdo.
       dMMMMMMMMMMMMMMMMMNh:
       +hMMMMMMMMMMMMMMMMMmy.
         -oNMMMMMMMMMMmy+.`
           `:yNMMMds/.`
              .//`
"#, main_color: 5, },
    ),

    (
        "rhel",
        Logo {
            logo: r#"
         .MMM..:MMMMMMM
          MMMMMMMMMMMMMMMMMM
          MMMMMMMMMMMMMMMMMMMM.
         MMMMMMMMMMMMMMMMMMMMMM
        ,MMMMMMMMMMMMMMMMMMMMMM:
        MMMMMMMMMMMMMMMMMMMMMMMM
  .MMMM'  MMMMMMMMMMMMMMMMMMMMMM
 MMMMMM    `MMMMMMMMMMMMMMMMMMMM.
MMMMMMMM      MMMMMMMMMMMMMMMMMM .
MMMMMMMMM.       `MMMMMMMMMMMMM' MM.
MMMMMMMMMMM.                     MMMM
`MMMMMMMMMMMMM.                 ,MMMMM.
 `MMMMMMMMMMMMMMMMM.          ,MMMMMMMM.
    MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
      MMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMMM:
         MMMMMMMMMMMMMMMMMMMMMMMMMMMMMM
            `MMMMMMMMMMMMMMMMMMMMMMMM:
                ``MMMMMMMMMMMMMMMMM'
"#, main_color: 1, },
    ),

    (
        "clear",
        Logo {
            logo: r#"
          BBB
       BBBBBBBBB
     BBBBBBBBBBBBBBB
   BBBBBBBBBBBBBBBBBBBB
   BBBBBBBBBBB         BBB
  BBBBBBBB{3}YYYYY
{4}  BBBBBBBB{3}YYYYYY
{4}  BBBBBBBB{3}YYYYYYY
{4}  BBBBBBBBB{3}YYYYY{7}W
{6} GG{4}BBBBBBBY{3}YYYY{7}WWW
{6} GGG{4}BBBBBBB{3}YY{7}WWWWWWWW
{6} GGGGGG{4}BBBBBB{7}WWWWWWWW
{6} GGGGGGGG{4}BBBB{7}WWWWWWWW
{6}GGGGGGGGGGG{4}BBB{7}WWWWWWW
{6}GGGGGGGGGGGGG{4}B{7}WWWWWW
{6}GGGGGGGG{7}WWWWWWWWWWW
{6}GG{7}WWWWWWWWWWWWWWWW
 WWWWWWWWWWWWWWWW
      WWWWWWWWWW
          WWW
"#, main_color: 4, },
    ),

    (
        "artix",
        Logo {
            logo: r#"
                  'o'
                 'ooo'
                'ooxoo'
               'ooxxxoo'
              'oookkxxoo'
             'oiioxkkxxoo'
            ':;:iiiioxxxoo'
               `'.;::ioxxoo'
          '-.      `':;jiooo'
         'oooio-..     `'i:io'
        'ooooxxxxoio:,.   `'-;'
       'ooooxxxxxkkxoooIi:-.  `'
      'ooooxxxxxkkkkxoiiiiiji'
     'ooooxxxxxkxxoiiii:'`     .i'
    'ooooxxxxxoi:::'`       .;ioxo'
   'ooooxooi::'`         .:iiixkxxo'
  'ooooi:'`                `'';ioxxo'
 'i:'`                          '':io'
'`                                   `'
"#, main_color: 4, },
    ),

    (
        "void",
        Logo {
            logo: r#"
               __.;=====;.__
            _.=+==++=++=+=+===;.
             -=+++=+===+=+=+++++=_
        .     -=:``     `--==+=++==.
       _vi,    `            --+=++++:
      .uvnvi.       _._       -==+==+.
     .vvnvnI`    .;==|==;.     :|=||=|.
{7}+QmQQm{2}pvvnv; {7}_yYsyQQWUUQQQm #QmQ#{2}:{7}QQQWUVQQm.
{7} -QQWQW{2}pvvo{7}wZ?.wQQQE{2}==<{7}QWWQ/QWQW.QQWW{2}(: {7}jQWQE
{7}  -$QQQQmmU'  jQQQ@{2}+=<{7}QWQQ)mQQQ.mQQQC{2}+;{7}jWQQ@'
{7}   -$WQ8Y{2}nI:   {7}QWQQwgQQWV{2}`{7}mWQQ.jQWQQgyyWW@!
{2}     -1vvnvv.     `~+++`        ++|+++
      +vnvnnv,                 `-|===
       +vnvnvns.           .      :=-
        -Invnvvnsi..___..=sv=.     `
          +Invnvnvnnnnnnnnvvnn;.
            ~|Invnvnvvnvvvnnv)+`
               -~|(*l)*|~
"#, main_color: 2, },
    ),

    (
        "opensuse-tumbleweed",
        Logo {
            logo: r#"
        ......
     .,cdxxxoc,.               .:kKMMMNWMMMNk:.
    cKMMN0OOOKWMMXo. ;        ;0MWk:.      .:OMMk.
  ;WMK;.       .lKMMNM,     :NMK,             .OMW;
 cMW;            'WMMMN   ,XMK,                 oMM'
.MMc               ..;l. xMN:                    KM0
'MM.                   'NMO                      oMM
.MM,                 .kMMl                       xMN
 KM0               .kMM0. .dl:,..               .WMd
 .XM0.           ,OMMK,    OMMMK.              .XMK
   oWMO:.    .;xNMMk,       NNNMKl.          .xWMx
     :ONMMNXMMMKx;          .  ,xNMWKkxllox0NMWk,
         .....                    .:dOOXXKOxl,
"#, main_color: 7, },
    ),

    (
        "peppermintos",
        Logo {
            logo: r#"
               PPPPPPPPPPPPPP
           PPPP{15}MMMMMMM{1}PPPPPPPPPPP
         PPPP{15}MMMMMMMMMM{1}PPPPPPPP{15}MM{1}PP
       PPPPPPPP{15}MMMMMMM{1}PPPPPPPP{15}MMMMM{1}PP
     PPPPPPPPPPPP{15}MMMMMM{1}PPPPPPP{15}MMMMMMM{1}PP
    PPPPPPPPPPPP{15}MMMMMMM{1}PPPP{15}M{1}P{15}MMMMMMMMM{1}PP
   PP{15}MMMM{1}PPPPPPPPPP{15}MMM{1}PPPPP{15}MMMMMMM{1}P{15}MM{1}PPPP
   P{15}MMMMMMMMMM{1}PPPPPP{15}MM{1}PPPPP{15}MMMMMM{1}PPPPPPPP
  P{15}MMMMMMMMMMMM{1}PPPPP{15}MM{1}PP{15}M{1}P{15}MM{1}P{15}MM{1}PPPPPPPPPPP
  P{15}MMMMMMMMMMMMMMMM{1}PP{15}M{1}P{15}MMM{1}PPPPPPPPPPPPPPPP
  P{15}MMM{1}PPPPPPPPPPPPPPPPPPPPPPPPPPPPPP{15}MMMMM{1}P
  PPPPPPPPPPPPPPPP{15}MMM{1}P{15}M{1}P{15}MMMMMMMMMMMMMMMM{1}PP
  PPPPPPPPPPP{15}MM{1}P{15}MM{1}PPPP{15}MM{1}PPPPP{15}MMMMMMMMMMM{1}PP
   PPPPPPPP{15}MMMMMM{1}PPPPP{15}MM{1}PPPPPP{15}MMMMMMMMM{1}PP
   PPPP{15}MM{1}P{15}MMMMMMM{1}PPPPPP{15}MM{1}PPPPPPPPPP{15}MMMM{1}PP
    PP{15}MMMMMMMMM{1}P{15}M{1}PPPP{15}MMMMMM{1}PPPPPPPPPPPPP
    PP{15}MMMMMMM{1}PPPPPPP{15}MMMMMM{1}PPPPPPPPPPPP
      PP{15}MMMM{1}PPPPPPPPP{15}MMMMMMM{1}PPPPPPPP
        PP{15}MM{1}PPPPPPPP{15}MMMMMMMMMM{1}PPPP
           PPPPPPPPPP{15}MMMMMMMM{1}PPPP
               PPPPPPPPPPPPPP
"#, main_color: 1, },
    ),

    (
        "gentoo",
        Logo {
            logo: r#"
{5}         -/oyddmdhs+:.
     -o{7}dNMMMMMMMMNNmhy+{5}-`
   -y{7}NMMMMMMMMMMMNNNmmdhy{5}+-
 `o{7}mMMMMMMMMMMMMNmdmmmmddhhy{5}/`
 om{7}MMMMMMMMMMMN{5}hhyyyo{7}hmdddhhhd{5}o`
.y{7}dMMMMMMMMMMd{5}hs++so/s{7}mdddhhhhdm{5}+`
 oy{7}hdmNMMMMMMMN{5}dyooy{7}dmddddhhhhyhN{5}d.
  :o{7}yhhdNNMMMMMMMNNNmmdddhhhhhyym{5}Mh
    .:{7}+sydNMMMMMNNNmmmdddhhhhhhmM{5}my
       /m{7}MMMMMMNNNmmmdddhhhhhmMNh{5}s:
    `o{7}NMMMMMMMNNNmmmddddhhdmMNhs{5}+`
  `s{7}NMMMMMMMMNNNmmmdddddmNMmhs{5}/.
 /N{7}MMMMMMMMNNNNmmmdddmNMNdso{5}:`
+M{7}MMMMMMNNNNNmmmmdmNMNdso{5}/-
yM{7}MNNNNNNNmmmmmNNMmhs+/{5}-`
/h{7}MMNNNNNNNNMNdhs++/{5}-`
`/{7}ohdmmddhys+++/:{5}.`
  `-//////:--.
"#, main_color: 5, },
    ),

    (
        "cachyos",
        Logo {
            logo: r#"
            {10}.{2}-------------------------:
          .{4}+={2}========================.
         :{4}++{2}==={4}++==={2}===============-       :{4}++{2}-
        :{4}*++{2}===={4}+++++=={2}===========-        .==:
       -{4}*+++{2}====={4}+***++={2}=========:
      ={4}*++++={2}=======------------:
     ={4}*+++++={2}====-                     {10}...{2}
   .{4}+*+++++{2}=-===:                    .{4}=+++={2}:
  :{4}++++{2}=====-==:                     -***{4}**{2}+
 :{4}++={2}=======-=.                      .=+**+{10}.{2}
.{4}+{2}==========-.                          {10}.{2}
 :{4}+++++++{2}====-                                {10}.{2}--==-{10}.{2}
  :{4}++{2}==========.                             {10}:{4}+++++++{2}{10}:
   {2}.-===========.                            =*****+*+
    {2}.-===========:                           .+*****+:
      {2}-======={4}++++{2}:::::::::::::::::::::::::-:  {10}.{2}---:
       :======{4}++++{2}===={4}+++******************=.
        {2}:====={4}+++{2}=========={4}++++++++++++++*-
         {2}.===={4}++{2}=============={4}++++++++++*-
          {2}.==={4}+{2}=================={4}+++++++:
           {2}.-======================={4}+++:
             {10}..........................
"#, main_color: 2, },
    ),

    (
        "bedrock",
        Logo {
            logo: r#"
--------------------------------------
--------------------------------------
--------------------------------------
---{7}\\\\\\\\\\\\{15}-----------------------
----{7}\\\\      \\\{15}----------------------
-----{7}\\\\      \\\{15}---------------------
------{7}\\\\      \\\\\\\\\\\\\\\\\{15}------
-------{7}\\\\                    \\\{15}-----
--------{7}\\\\                    \\\{15}----
---------{7}\\\\        ______      \\\{15}---
----------{7}\\\\                   ///{15}---
-----------{7}\\\\                 ///{15}----
------------{7}\\\\               ///{15}-----
-------------{7}\\\\////////////////{15}------
--------------------------------------
--------------------------------------
--------------------------------------
"#, main_color: 15, },
    ),

    (
        "linux",
        Logo {
            logo: r#"
{0}        #####
{0}       #######
{0}       ##{15}O{0}#{15}O{0}##
{0}       #{3}#####{15}#
{0}     ##{15}##{3}###{15}##{0}##
{0}    #{15}##########{0}##
{0}   #{15}############{0}##
{0}   #{15}############{0}###
{3}  ##{0}#{15}###########{0}##{3}#
{3}######{0}#{15}#######{0}#{3}######
{3}#######{0}#{15}#####{0}#{3}#######
{3}  #####{0}#######{3}#####
"#, main_color: 15, },
    ),

    (
        "mandriva",
        Logo {
            logo: r#"
{3}                        ``
{3}                       `-.
{4}      `               {3}.---
{4}    -/               {3}-::--`
{4}  `++    {3}`----...```-:::::.
{4} `os.      {3}.::::::::::::::-```     `  `
{4} +s+         {3}.::::::::::::::::---...--`
{4}-ss:          {3}`-::::::::::::::::-.``.``
{4}/ss-           {3}.::::::::::::-.``   `
{4}+ss:          {3}.::::::::::::-
{4}/sso         {3}.::::::-::::::-
{4}.sss/       {3}-:::-.`   .:::::
{4} /sss+.    {3}..`{4}  `--`    {3}.:::
{4}  -ossso+/:://+/-`        {3}.:`
{4}    -/+ooo+/-.              {3}`

"#, main_color: 4, },
    ),

    (
        "GNU",
        Logo {
            logo: r#"
          _-`````-,           ,- '- .
  .'   .- - |          | - -.  `.
 /.'  /                     `.   \
:/   :      _...   ..._      ``   :
::   :     /._ .`:'_.._\.    ||   :
::    `._ ./  ,`  :    \ . _.''   .
`:.      /   |  -.  \-. \\_      /
  \:._ _/  .'   .@)  \@) ` `\ ,.'
     _/,--'       .- .\,-.`--`.
       ,'/''     (( \ `  )
        /'/'  \    `-'  (
         '/''  `._,-----'
          ''/'    .,---'
           ''/'      ;:
             ''/''  ''/
               ''/''/''
                 '/'/'
                  `;
"#, main_color: 7, },
    ),

    (
        "freebsd",
        Logo {
            logo: r#"
{7}   ```                    {1}..,'`
  {7}` `.....---...{1}....--.```   -/
  {7}+o   .--`         {1}/y:`      +.
   {7}yo`:.            {1}:o      `+-
    {7}y/               {1}-/`   -o/
   {7}.-                  {1}::/sy+:.
   {7}/                     {1}`--  /
  {7}`:                          {1}:`
  {7}`:                          {1}:`
   {7}/                          {1}/
   {7}.-                        {1}-.
    {7}--                      {1}-.
     {7}`:`                  {1}`:`
       .--             `--.
          .---.....----.
"#, main_color: 1, },
    ),

    (
        "devuan",
        Logo {
            logo: r#"
    ..,,;;;::;,..
           `':ddd;:,.
                 `'dPPd:,.
                     `:b$$b`.
                        'P$$$d`
                         .$$$$$`
                         ;$$$$$P
                      .:P$$$$$$`
                  .,:b$$$$$$$;'
             .,:dP$$$$$$$$b:'
      .,:;db$$$$$$$$$$Pd'`
 ,db$$$$$$$$$$$$$$b:'`
:$$$$$$$$$$$$b:'`
 `$$$$$bd:''`
   `'''`
"#, main_color: 5, },
    ),

    (
        "fallback",
        Logo {
            logo: r#"{7}
      ________
  _jgN########Ngg_
_N##N@@""  ""9NN##Np_
d###P            N####p
"^^"              T####
                d###P
            _g###@F
            _gN##@P
         gN###F"
         d###F
         0###F
         0###F
         0###F
         "NN@'

          ___
         q###r
          ---
"#, main_color: 7, },
    ),
];


pub fn colorize_logo(logo: &str) -> String {
    let mut result = logo.to_string();

    for index in 0..16 {
        let placeholder = format!("{{{index}}}");
        result = result.replace(
            &placeholder,
            colors::get_color(index, true),
        );
    }

    result
}

pub fn get_logos_values(key: &str) -> (String, String) {
    let key = if key.is_empty() {
        distro_id()
    } else {
        key.to_string()
    };

    let (_, logo) = LOGOS
        .iter()
        .find(|(id, _)| *id == key)
        .or_else(|| LOGOS.iter().find(|(id, _)| *id == "fallback"))
        .unwrap();

    (
        colorize_logo(logo.logo),
        colors::get_color(logo.main_color, true).to_string(),
    )
}
