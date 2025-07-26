# Configuração do Postman para Learn Rust CRUD API

Este guia explica como configurar o Postman para testar a API CRUD em Rust.

## Arquivos Fornecidos

1. `postman_collection.json` - Coleção com todas as requisições
2. `postman_environment.json` - Ambiente de desenvolvimento local
3. `postman_environment_production.json` - Ambiente de produção (Railway)

## Como Importar

### 1. Importar a Coleção
1. Abra o Postman
2. Clique em "Import" (botão no canto superior esquerdo)
3. Arraste o arquivo `postman_collection.json` ou clique em "Upload Files"
4. Selecione o arquivo e clique em "Import"

### 2. Importar os Ambientes
1. Clique em "Import" novamente
2. Arraste os arquivos `postman_environment.json` e `postman_environment_production.json` ou clique em "Upload Files"
3. Selecione os arquivos e clique em "Import"

### 3. Selecionar o Ambiente
1. No canto superior direito do Postman, clique no dropdown de ambiente
2. Selecione:
   - **"Learn Rust CRUD Development Environment"** para desenvolvimento local
   - **"Learn Rust CRUD Production Environment"** para produção

## Estrutura da Coleção

### Authentication
- **Login**: POST `/auth/login` - Faz login e retorna tokens
- **Refresh Token**: POST `/auth/refresh` - Renova o access token
- **Logout**: POST `/auth/logout` - Faz logout

### CRUD Operations
- **Create Data**: POST `/data` - Cria novo registro
- **Read All Data**: GET `/data` - Lista todos os registros
- **Read Data by ID**: GET `/data/:id` - Busca registro específico
- **Update Data**: PUT `/data/:id` - Atualiza registro
- **Delete Data**: DELETE `/data/:id` - Remove registro

## Como Usar

### 1. Iniciar o Servidor (Desenvolvimento Local)
```bash
cargo run
```

**Para Produção:**
A API já está disponível em: https://learn-rust-crud-production.up.railway.app

### 2. Fazer Login
1. Execute a requisição "Login" na pasta "Authentication"
2. Use um dos usuários disponíveis:
   - `admin/admin123`
   - `user1/password123`
   - `user2/password456`
3. O access_token e refresh_token serão salvos automaticamente nas variáveis

### 3. Testar Operações CRUD
1. **Criar dados**: Execute "Create Data" com um JSON como:
   ```json
   {
       "data1": ["texto1", "texto2", "texto3"],
       "data2": [1, 2, 3, 4, 5]
   }
   ```

2. **Listar dados**: Execute "Read All Data" para ver todos os registros

3. **Buscar por ID**: Copie um ID da resposta anterior e atualize a variável `data_id` no ambiente

4. **Atualizar dados**: Execute "Update Data" com novos dados

5. **Deletar dados**: Execute "Delete Data" para remover um registro

## Variáveis do Ambiente

### Desenvolvimento Local
- `base_url`: http://127.0.0.1:8080
- `access_token`: Token de acesso JWT (preenchido automaticamente após login)
- `refresh_token`: Token de renovação (preenchido automaticamente após login)
- `data_id`: ID do registro para operações específicas

### Produção
- `base_url`: https://learn-rust-crud-production.up.railway.app
- `access_token`: Token de acesso JWT (preenchido automaticamente após login)
- `refresh_token`: Token de renovação (preenchido automaticamente após login)
- `data_id`: ID do registro para operações específicas

## Scripts Automáticos

A coleção inclui scripts que:
- Capturam automaticamente os tokens após login
- Salvam os tokens nas variáveis do ambiente
- Permitem usar os tokens em requisições subsequentes

## Exemplos de Uso

### Fluxo Completo de Teste

1. **Login** → Recebe tokens
2. **Create Data** → Cria um registro
3. **Read All Data** → Lista todos os registros
4. **Read Data by ID** → Busca o registro criado (copie o ID da resposta anterior)
5. **Update Data** → Atualiza o registro
6. **Delete Data** → Remove o registro
7. **Logout** → Invalida os tokens

### Renovação de Token

Se o access_token expirar:
1. Execute "Refresh Token" usando o refresh_token salvo
2. O novo access_token será salvo automaticamente
3. Continue testando as operações CRUD

## Troubleshooting

### Erro 401 Unauthorized
- Verifique se o access_token está válido
- Execute "Refresh Token" se necessário
- Verifique se o token está sendo enviado no header Authorization

### Erro 404 Not Found
- Verifique se o servidor está rodando em http://127.0.0.1:8080
- Verifique se a URL está correta

### Erro 400 Bad Request
- Verifique o formato do JSON enviado
- Verifique se todos os campos obrigatórios estão presentes

## Configuração do Servidor

Certifique-se de que o arquivo `.env` está configurado corretamente:

```env
SERVER_ADDR=127.0.0.1:8080
JWT_SECRET=your-secret-key-change-in-production
JWT_ISSUER=learn-rust-crud
ACCESS_TOKEN_EXPIRATION_HOURS=1
REFRESH_TOKEN_EXPIRATION_DAYS=30
```

## Usuários de Teste Disponíveis

- **admin/admin123** - Usuário administrador
- **user1/password123** - Usuário comum 1
- **user2/password456** - Usuário comum 2

Cada usuário tem acesso apenas aos seus próprios dados. 