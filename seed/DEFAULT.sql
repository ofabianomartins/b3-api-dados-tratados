INSERT INTO calendars(name, code) VALUES ('ANBIMA', 'anbima');
INSERT INTO calendars(name, code) VALUES ('b3', 'b3');


INSERT INTO indicators(name, symbol, description) VALUES ('Preço/Lucro', 'P/L', 'Preço sobre o patrimônio');
INSERT INTO indicators(name, symbol, description) VALUES ('Preço/Preço/Valor Patrimonial por Ação', 'P/VPA', 'Preço/Valor Patrimonial por Ação');
INSERT INTO indicators(name, symbol, description) VALUES ('Return Over Investiment', 'ROE', 'Retorno sobre o investimento');
INSERT INTO indicators(name, symbol, description) VALUES ('EV/Ebitda', 'EV/Ebitda', 'Retorno sobre o investimento');

INSERT INTO currencies(name, code) VALUES ('Real Brasileiro', 'BRL');

INSERT INTO companies(name, company_type) VALUES ('Petrolio Brasileiro', 'DEFAULT');

INSERT INTO tickers(symbol, security_type, creation_date, company_id, currency_id, calendar_id) VALUES ( 
      'PETR4', 
      'STOCK', 
      '2023-01-01',
      (SELECT id from companies WHERE name='Petrolio Brasileiro'),
      (SELECT id from currencies WHERE code='BRL'),
      (SELECT id from calendars WHERE code='b3')
);



