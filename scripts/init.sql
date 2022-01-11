insert into users(name) VALUES ('Hayao');

insert into skill(title, content, userid) VALUES ('Rust', 'Webフロントエンド、Webバックエンド, CLI, システムプログラミングまでなんでもござれな言語。自分の中では一番使い慣れている言語', 1);
insert into skill(title, content, userid) VALUES ('Elm', 'Webフロントエンド専用機。Elmアーキテクチャという絶対の戒律があるので書きやすい。Webフロントエンドを書くなら第一候補になる言語', 1);
insert into skill(title, content, userid) VALUES ('Haskell', '勉強していて一番楽しい言語。現状全然使えていないが、第3の言語にしたい。使い所はWebバックエンドかな？', 1);
insert into skill(title, content, userid) VALUES ('SQL', '基本的な構文は知っている。パフォーマンスチューニングの経験はゼロ', 1);

insert into career(name, years_from, years_to, description, userid) VALUES('ピーマン大学院', '2019-04-01', '2021-03-31', 'ピーマンの魅力に魅せられ、ピーマンの栽培方法の研究をしていた。', 1);
insert into career(name, years_from, years_to, description, userid) VALUES('ピーマン大学院', '2021-04-01', NULL, 'ピーマン管理システムの構築をしている。この管理システムを使うと甘いピーマンを安定して生産することができる。', 1);


insert into about(description, userid) VALUES ('システム開発会社で働く週休３日を熱望するしがないシステムエンジニア。エクセルを多用するが詳しくない。SEとして1~3年経験を積んだあとはITエンジニアとして働いてたい。', 1);