# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule Lili.Repo.Migrations.CreateUsers do
  use Ecto.Migration

  def change do
    execute "CREATE EXTENSION IF NOT EXISTS citext", ""
    
    create table(:users, primary_key: false) do
      add :id, :binary_id, primary_key: true
      add :email, :citext, null: false
      add :username, :citext, null: false
      add :hashed_password, :string, null: false

      timestamps()
    end

    create unique_index(:users, [:username])
    create unique_index(:users, [:email])
  end
end
