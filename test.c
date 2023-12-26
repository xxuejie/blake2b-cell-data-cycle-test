#include <ckb_syscalls.h>
#include <blake2b.h>
#include <molecule/blockchain.h>

#define ONE_BATCH_SIZE 32768
#define SCRIPT_SIZE 32768

int load_and_hash_cell_data(blake2b_state *ctx, size_t start, size_t index,
                            size_t source) {
  int err = CKB_SUCCESS;
  uint8_t temp[ONE_BATCH_SIZE];
  uint64_t len = ONE_BATCH_SIZE;
  err = ckb_load_cell_data(temp, &len, start, index, source);
  if (err != CKB_SUCCESS) {
    return err;
  }

  blake2b_update(ctx, (char *)&len, sizeof(uint32_t));

  uint64_t offset = (len > ONE_BATCH_SIZE) ? ONE_BATCH_SIZE : len;
  blake2b_update(ctx, temp, offset);
  while (offset < len) {
    uint64_t current_len = ONE_BATCH_SIZE;
    err = ckb_load_cell_data(temp, &current_len, start + offset, index, source);
    if (err != CKB_SUCCESS) {
      return err;
    }
    uint64_t current_read =
        (current_len > ONE_BATCH_SIZE) ? ONE_BATCH_SIZE : current_len;
    blake2b_update(ctx, temp, current_read);
    offset += current_read;
  }

  return CKB_SUCCESS;
}

int main() {
  int err = CKB_SUCCESS;

  uint8_t script[SCRIPT_SIZE];
  uint64_t script_len = SCRIPT_SIZE;
  err = ckb_load_script(script, &script_len, 0);
  if (err != CKB_SUCCESS) {
    return err;
  }

  mol_seg_t script_seg;
  script_seg.ptr = (uint8_t *)script;
  script_seg.size = script_len;

  mol_seg_t args_seg = MolReader_Script_get_args(&script_seg);
  mol_seg_t args_bytes_seg = MolReader_Bytes_raw_bytes(&args_seg);
  if (args_bytes_seg.size != 32) {
    ckb_debug("Invalid script args length!");
    return 1;
  }

  blake2b_state state;
  ckb_blake2b_init(&state, 32);

  load_and_hash_cell_data(&state, 0, 0, CKB_SOURCE_GROUP_INPUT);

  uint8_t result[32];
  blake2b_final(&state, result, 32);

  if (memcmp(result, args_bytes_seg.ptr, 32) != 0) {
    ckb_debug("Invalid hash");
    return 1;
  }

  return 0;
}
