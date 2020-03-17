module BinaryTreeTraversal
  ( preOrder
  , inOrder
  , postOrder
  ) where

    -- import BinaryTreeTraversal.Types

    data Tree a = Nil | Node (Tree a) a (Tree a)

    -- 1.) Root node, 2.) traverse left subtree, 3.) traverse right subtree.
    preOrder :: Tree a -> [a]
    preOrder Nil = []
    preOrder (Node lc val rc) = val : (preOrder lc) ++ (preOrder rc)

    -- 1.) Traverse left subtree, 2.) root node, 3.) traverse right subtree.
    inOrder :: Tree a -> [a]
    inOrder Nil = []
    inOrder (Node lc val rc) = (inOrder lc) ++ [val] ++ (inOrder rc)

    -- 1.) Traverse left subtree, 2.) traverse right subtree, 3.) root node.
    postOrder :: Tree a -> [a]
    postOrder Nil = []
    postOrder (Node lc val rc) = (postOrder lc) ++ (postOrder rc) ++ [val]
