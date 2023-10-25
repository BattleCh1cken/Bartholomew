create table if not exists teams
(
  id integer primary key autoincrement not null,
  name varchar not null,
  leader bigint not null,
  guild bigint, 
  foreign key(guild) references guilds(id)
);

create table if not exists guilds 
(
  id bigint primary key not null,
  unique(id)
);

create table if not exists users
(
  id bigint primary key not null,
  unique(id)
);

create table if not exists scores 
(
  id integer primary key autoincrement not null,
  score integer not null,
  user bigint not null, 
  team bigint not null,
  foreign key(user) references users(id),
  foreign key(team) references teams(id),
  unique(user, team)
);
