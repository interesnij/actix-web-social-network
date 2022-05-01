-- создать базу
CREATE DATABASE my_database;

-- подключиться к базе
\connect my_database;

-- создать таблицу my_table с полями field1 (тип целочисленный, обязательное для заполнения), field2 (тип строка 255 символов)
CREATE TABLE my_table (field1 INT NOT NULL, field2 VARCHAR(255));

-- вывести все таблицы
\d

-- удалить таблицу my_table
DROP TABLE my_table;

-- внести в таблицу запись
INSERT INTO my_table(field1, field2) VALUES(1,'Any text value');

-- вывести записи
SELECT * FROM my_table; -- все записи
SELECT * FROM my_table WHERE field1 = 1; -- все, где field1 = 1
SELECT * FROM my_table WHERE field1 != 1; -- и т д
SELECT * FROM my_table WHERE field1 > 1;
SELECT * FROM my_table LIMIT 100; -- первые 100 записей;
SELECT * FROM my_table LIMIT 100 OFFSET 200; -- запись с 201 по 300;

-- сортировка при выводе
SELECT * FROM my_table ORDER BY field1 ASC; -- вывести отсортировав в возрастающем порядке
SELECT * FROM my_table ORDER BY field1 DESC; -- вывести отсортировав в убывающем порядке

-- изменить запись таблицы (поле field2 строки, где field1 = 1);
UPDATE my_table SET field2 = 'Other text value' WHERE field1 = 1;

-- удаление данных
DELETE FROM my_table; -- удалить все записи;
DELETE FROM my_table WHERE field1 = 1; -- удалить запись где field1 = 1;


-- ***************************************
-- нормализация (разбиение таблиц на несколько)
-- ***************************************

-- Constraints - ограничения типов данных
CREATE TABLE my_table (
  field1 INT NOT NULL, -- запись обязательна
  field2 VARCHAR(255) NOT NULL UNIQUE, -- запись должна быть уникальной
  field3 BOOLEAN NOT NULL DEFAULT TRUE -- значение по умолчанию - true
  ...
);

-- Первичный и внешние ключи
-- при создании записи таблицы с отсутствующим внешним ключом выведется запись об ошибке. будут выводится ошибки и в иных случаях, когда будут нарушаться связи.
CREATE TABLE IF NOT EXISTS my_table ( -- ключ IF NOT EXISTS проверяет, существует ли таблица.
  field1 SERIAL INT PRIMARY KEY, -- при добавлении PRIMARY KEY поле автоматически наследует ограничения NOT NULL и UNIQUE, и создается индекс. SERIAL тип данных являющийся автоматически увеличивающимся счетчиком (аналог ключа AUTOINCREMENT в Sqlite)
  field2 VARCHAR(255) NOT NULL UNIQUE,
  field3 INT NOT NULL,
  FOREIGN KEY(field3) REFERENCES other_table(field_name) -- поле ссылается на внешнюю таблицу other_table на поле field_name, которое обязательно должно быть с PRIMARY KEY
);

-- вывод данных из нескольких таблиц со связанными полями
SELECT * FROM table_1 LEFT JOIN table_2 ON (table_2.field = table_1.field);

-- алиасы, нужны для удобства. Также, при выводе наименование таблиц или полей выводится алиасом, при его наличии.
SELECT * FROM table_1 as tab1 LEFT JOIN table_2 as tab2 ON (tab1.field = tab2.field);


-- ***************************************
-- Редактирование таблиц, расширенные возможности SELECT, функции
-- ***************************************

-- Добавление поля в таблицу
ALTER TABLE user_profiles ADD COLUMN survey
INT NOT NULL DEFAULT 0;

-- добавление поля с автоинкрементом и primary key в таблицу
ALTER TABLE new_friends_perms ADD COLUMN can_create_post "char";

-- Удаление поля из таблицы
ALTER TABLE new_friends_perms DROP COLUMN can_copy_post;

-- переименовать поле
ALTER TABLE table_name RENAME old_field TO new_field;

-- сменить тип данных
ALTER TABLE posts ALTER COLUMN position SET
DATA TYPE SMALLINT;

-- изменить значение по умолчанию
ALTER TABLE table_name ALTER COLUMN any_field SET
DEFAULT 'new value';

-- добавить/удалить constraint NOT NULL
ALTER TABLE table_name ALTER COLUMN any_field
SET|DROP NOT NULL;

-- переименовать таблицу
ALTER TABLE table_name RENAME TO new_table_name;

-- Расширенные возможности SELECT
SELECT * FROM table WHERE field1 LIKE 'value'; -- field1 = 'value'
SELECT * FROM table WHERE field1 LIKE 'val%'; -- field1 начинается с 'val'
SELECT * FROM table WHERE field1 LIKE '%lue'; -- field1 заканчивается на 'lue'
SELECT * FROM table WHERE field1 LIKE '%e%'; -- field1 содержит 'e'
-- несколько условий
SELECT * FROM table WHERE field1 = 'value' AND field2 > 'value2';
SELECT * FROM table WHERE field1 = 'value' OR field2 > 'value2';

-- вывод уникальных записей
SELECT DISTINCT field1 FROM table;

-- группирование записей

SELECT field1, COUNT(field1) FROM table GROUP BY field1;
-- сгруппирует записи таблицы table по полю field и выведет уникальные значения field и количество повторений

SELECT field1, COUNT(field1) FROM table GROUP BY field1
HAVING COUNT(field) > 3;
-- сгруппирует записи таблицы table по полю field и выведет уникальные значения field и количество повторений, где количество повторений больше 3
