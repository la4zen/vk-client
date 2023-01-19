import { invoke } from "@tauri-apps/api";
import { useState } from "react";

export const Auth = () => {
  return (
    <div>
      <button
        onClick={() => {
          invoke("vk_authorize").then(res => {
            console.log(res)
          }).catch(err => {
            console.log(err)
          })
        }}
      >
        Войти
      </button>
    </div>
  );
};
