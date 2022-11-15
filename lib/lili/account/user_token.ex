# Copyright © 2022 Emmy Emmycelium <https://cybele.dev/>
# SPDX-License-Identifier: AGPL-3.0-only
defmodule Lili.Account.UserToken do
  use Ecto.Schema
  import Ecto.Query

  # It is very important to keep the reset password token expiry short,
  # since someone with access to the e-mail may take over the account.
  @reset_password_validity_in_days 1
  @confirm_validity_in_days 7
  @change_email_validity_in_days 7
  @session_validity_in_days 60

  @hash_algorithm :sha256
  @rand_size 32
  
  schema "user_tokens" do
    field :token, :binary
    field :context, :string
    belongs_to :user, Lili.Account.User, foreign_key: :user_id, type: :binary_id

    timestamps(updated_at: false)
  end

  @doc """
  Builds a URL base64 encoded token with a hashed counterpart. The hashed
  version should be persisted in the database and the unhashed version should
  be given to the client.
  """
  def build_token(user, context) do
    token = :crypto.strong_rand_bytes(@rand_size)
    hashed_token = :crypto.hash(@hash_algorithm, token)

    {Base.url_encode64(token, padding: false),
      %Lili.Account.UserToken{
        token: hashed_token,
        context: context,
        user_id: user.id
    }}
  end
  
  @doc """
  Checks if a token is valid base64 and returns its lookup query.
  """
  def verify_token_query(token, context) do
    case Base.url_decode64(token, padding: false) do
      {:ok, decoded_token} ->
        hashed_token = :crypto.hash(@hash_algorithm, decoded_token)
        days = days_for_context(context)

        query =
          from token in token_and_context_query(hashed_token, context),
            join: user in assoc(token, :user),
            where: token.inserted_at > ago(^days, "day"),
            select: user

        {:ok, query}
      
      :error ->
        :error
    end
  end

  defp days_for_context("session"), do: @session_validity_in_days
  defp days_for_context("confirm"), do: @confirm_validity_in_days
  defp days_for_context("reset_password"), do: @reset_password_validity_in_days
  defp days_for_context("change_email"),   do: @change_email_validity_in_days

  @doc """
  Returns the given token with the given context.
  """
  def token_and_context_query(token, context) do
    from Lili.Account.UserToken, where: [token: ^token, context: ^context]
  end

  @doc """
  Gets all tokens for the given user for the given contexts.
  """
  def user_and_contexts_query(user, :all) do
    from t in Lili.Account.UserToken, where: t.user_id == ^user.id
  end

  def user_and_contexts_query(user, [_ | _] = contexts) do
    from t in Lili.Account.UserToken, where: t.user_id == ^user.id and t.context in ^contexts
  end

end
