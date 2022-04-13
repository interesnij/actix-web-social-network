-- добавляем поле в таблицу
ALTER TABLE serve
ADD is_default boolean not null default false;

-- удаляем таблицу
DROP TABLE dbo.PurchaseOrderDetail;

-- переименовываем поле
ALTER TABLE smiles RENAME COLUMN category_id TO smile_categories_id;
ALTER TABLE stickers RENAME COLUMN category_id TO sticker_categories_id;

-- меняем тип поля
ALTER TABLE Customers
ALTER COLUMN FirstName TYPE VARCHAR(50);

-- переименовываем таблицу
ALTER TABLE table_name RENAME TO new_table_name;

-- удаляем ключ один ко многим
Alter table название таблицы drop constraint ключ;
