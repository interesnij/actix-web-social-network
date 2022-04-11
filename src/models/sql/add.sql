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
