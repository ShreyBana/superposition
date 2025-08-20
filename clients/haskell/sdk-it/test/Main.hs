{-# LANGUAGE OverloadedStrings #-}

module Main (main) where

import Data.Text qualified as T
import Io.Superposition.SuperpositionClient qualified as SDK
import Network.URI qualified as URI
import Prelude
import qualified Network.HTTP.Client as HTTP
import qualified Test.Hspec as Test
import qualified Io.Superposition.Command.ListDimensions as CMD
import qualified Io.Superposition.Model.ListDimensionsInput as CMD
import Test.Hspec (describe, it)
import Test.HUnit (assert)
import Data.Either

expectRight :: Either a b -> b
expectRight (Right b) = b
expectRight _ = undefined

expectJust :: Maybe a -> a
expectJust (Just a) = a
expectJust _ = undefined

mkClient :: HTTP.Manager -> SDK.SuperpositionClient
mkClient manager = expectRight $ SDK.build $ do
  SDK.setToken "some-token"
  SDK.setEndpointuri $ expectJust $ URI.parseURI "http://localhost:8080"
  SDK.setHttpmanager manager

-- newtype TestEnv = TestEnv {client :: SDK.SuperpositionClient}

listDimensions client = do
   output <- CMD.listDimensions client $ do
      CMD.setOrgId "localorg"
      CMD.setWorkspaceId "test"
   assert (isRight output)
   pure ()

main :: IO ()
main = do
   manager <- HTTP.newManager HTTP.defaultManagerSettings
   let client = mkClient manager
   Test.hspec $ Test.beforeAll (pure client) $ do
      describe "Dimensions API" $ do
         it "List Dimensions" listDimensions
