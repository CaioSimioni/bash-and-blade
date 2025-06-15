# Guia de Contribuição

Obrigado por querer contribuir com o Bash and Blade!

## Como contribuir

1. Faça um fork do projeto
2. Crie uma branch para sua feature ou correção (`git checkout -b minha-feature`)
3. Faça commits claros e objetivos
4. Antes de enviar um PR, rode o script `pre-commit.sh` para garantir que tudo está funcionando:
   - Testes automatizados
   - Build do projeto
   - Lints
5. Abra um Pull Request explicando suas mudanças

## Dicas

- Siga o padrão de código do projeto
- Sempre adicione testes para novas funcionalidades
- Atualize a documentação se necessário

Qualquer dúvida, abra uma issue!

---

## Após o PR ser aprovado

Após seu Pull Request ser aprovado e mesclado na branch `master`, siga este fluxo para manter a branch `dev` atualizada:

```sh
# Troque para a branch master e atualize
git checkout master
git pull github master

# Volte para a branch dev
git checkout dev

# Mescle as mudanças da master na dev
git merge master

# Atualize a dev no repositório remoto
git pull github dev
git push github dev
```

**Por que fazer isso?**  
Esse processo garante que a branch `dev` sempre contenha as últimas correções e funcionalidades já aprovadas na `master`, evitando conflitos futuros e mantendo o desenvolvimento organizado.
