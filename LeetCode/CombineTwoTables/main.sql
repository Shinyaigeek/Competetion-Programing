-- SELECT City, Status FROM Address WHERE PersonId IN (
--     SELECT DISTINCT(PersonId)
--     FROM Person
-- );

SELECT FirstName, LastName, City, Status From Address INNER JOIN Person ON Address.PersonId = Person.PersonId;