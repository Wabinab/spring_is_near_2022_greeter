class StaticPagesController < ApplicationController
  def home
    @account_id = params[:account_id] if params[:account_id]
    @public_key = params[:public_key] if params[:public_key]
    @all_keys = params[:all_keys] if params[:all_keys]
  end
end
