-- добавляем поле в таблицу
ALTER TABLE serve
ADD is_default boolean not null default false;

-- удаляем таблицу
DROP TABLE dbo.PurchaseOrderDetail;
