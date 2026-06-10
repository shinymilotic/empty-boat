DROP TABLE IF EXISTS article_tags;
DROP TABLE IF EXISTS followings;
DROP TABLE IF EXISTS comments;
DROP TABLE IF EXISTS favorites;
DROP TABLE IF EXISTS tags;
DROP TABLE IF EXISTS articles;
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users(
    id BIGINT generated always as identity PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    bio TEXT,
    image TEXT,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);


CREATE TABLE IF NOT EXISTS articles (
    id BIGINT generated always as identity PRIMARY KEY,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    description TEXT,
    slug VARCHAR(255) NOT NULL UNIQUE,
    author_id  INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_author FOREIGN KEY(author_id) REFERENCES users(id)
);


CREATE TABLE IF NOT EXISTS tags (
    id BIGINT generated always as identity primary KEY,
    name VARCHAR(255) unique NOT NULL
);

CREATE TABLE IF NOT EXISTS article_tags (
    article_id BIGINT NOT NULL,
    tag_id BIGINT not null,
    primary key(article_id, tag_id),
    constraint fk_article foreign key(article_id) references articles(id) on delete cascade ,
    constraint fk_tag foreign key(tag_id) references tags(id) on delete cascade
);


CREATE TABLE IF NOT EXISTS followings (
    following_id BIGINT not null,
    follower_id BIGINT not null,
    followed_on TIMESTAMP not null default now(),
    primary key (following_id, follower_id),
    constraint fk_following foreign key(following_id) references users(id),
    constraint fk_follower foreign key(follower_id) references users(id)
);



CREATE TABLE IF NOT EXISTS comments (
   id BIGINT generated always as identity PRIMARY KEY,
   article_id int not null,
   author_id int not null,
   body text not null,
   created_at TIMESTAMP not null default now(),
   constraint fk_article foreign key(article_id) references articles(id) on delete cascade,
   constraint fk_author foreign key(author_id) references users(id) on delete cascade
);


CREATE TABLE IF NOT EXISTS favorites (
    article_id BIGINT not null,
    user_id BIGINT not null,
    created_at TIMESTAMP not null default now(),
    primary key(article_id, user_id),
    constraint fk_article foreign key(article_id) references articles(id) on delete cascade,
    constraint fk_user foreign key(user_id) references users(id) on delete cascade
);
