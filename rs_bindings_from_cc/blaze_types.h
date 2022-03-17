// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_RS_BINDINGS_FROM_CC_BLAZE_TYPES_H_
#define CRUBIT_RS_BINDINGS_FROM_CC_BLAZE_TYPES_H_

#include <string>

#include "rs_bindings_from_cc/util/string_type.h"

namespace rs_bindings_from_cc {

// Representation of a Blaze label (for example //foo/bar:baz).
CRUBIT_DEFINE_STRING_TYPE(BlazeLabel);

}  // namespace rs_bindings_from_cc

#endif  // CRUBIT_RS_BINDINGS_FROM_CC_BLAZE_TYPES_H_
